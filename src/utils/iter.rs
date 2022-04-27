pub struct Chunks<I: Iterator>(I, usize);
impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut take: Vec<I::Item> = Vec::new();
        for _ in 0..self.1 {
            self.0.next().map(|n| take.push(n));
        }
        if take.is_empty() {
            None
        } else {
            Some(take)
        }
    }
}
pub struct StepBy<I: Iterator>(I, usize);
impl<I: Iterator> Iterator for StepBy<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..(self.1 - 1) {
            self.0.next();
        }
        self.0.next()
    }
}
pub struct SkipEvery<I: Iterator>(std::iter::Enumerate<I>, usize);
impl<I: Iterator> Iterator for SkipEvery<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next().map(|(i, n)| {
            if (i + 1) % self.1 == 0 {
                self.0.next().map(|(_, n2)| n2)
            } else {
                Some(n)
            }
        }) {
            Some(x) => x,
            None => None,
        }
    }
}
pub trait IterAddons: Iterator {
    fn to_chunks(self, chunk_size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks(self, chunk_size)
    }
    fn skip_every(self, n: usize) -> SkipEvery<Self>
    where
        Self: Sized,
    {
        SkipEvery(self.enumerate(), n)
    }
    fn step(self, n: usize) -> StepBy<Self>
    where
        Self: Sized,
    {
        StepBy(self, n)
    }
}

impl<I: Iterator> IterAddons for I {}

#[cfg(test)]
mod tests {
    use super::IterAddons;

    #[test]
    fn chunks() {
        let mut in_threes = (0..10).to_chunks(3);
        assert_eq!(in_threes.next(), Some(vec![0, 1, 2]));
        assert_eq!(in_threes.next(), Some(vec![3, 4, 5]));
        assert_eq!(in_threes.next(), Some(vec![6, 7, 8]));
        assert_eq!(in_threes.next(), Some(vec![9]));
        assert_eq!(in_threes.next(), None);
    }

    #[test]
    fn skip_every() {
        let mut skip_threes = (1..10).skip_every(3);
        assert_eq!(skip_threes.next(), Some(1));
        assert_eq!(skip_threes.next(), Some(2));
        assert_eq!(skip_threes.next(), Some(4));
        assert_eq!(skip_threes.next(), Some(5));
        assert_eq!(skip_threes.next(), Some(7));
        assert_eq!(skip_threes.next(), Some(8));
        assert_eq!(skip_threes.next(), None);
    }
}
