use crate::utils::solution::Solution;
extern crate ndarray;
use ndarray::prelude::*;
pub struct Day11Solution(Array2<u8>);
fn inc_all<'a, I: Iterator<Item = &'a mut u8>>(iter: I) {
    iter.for_each(|energy| *energy += 1);
}
impl Day11Solution {
    pub fn age_all(&mut self) {
        inc_all(self.0.iter_mut());
    }
    pub fn day(&mut self) {
        self.age_all();
        // dbg!(&self.0);
    }
}
impl Solution for Day11Solution {
    type Answer = usize;

    fn num() -> usize {
        11
    }
    fn units() -> String {
        "flashes".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        (0..10).for_each(|_| self.day());
        // let point:GridIndex<2> = [1,2].into();
        // let count = point.floodfill(|i|{
        //     self.0.contains_index(i) && self.0[i.clone()]<9
        //     // self.0.contains_index(i)
        // })
        // // .for_each(|d|{
        // //     dbg!(d);
        // // });
        // .count();
        // dbg!(count);
        0
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        0
    }
    fn solve(input: String) -> Self {
        let rows = input.matches('\n').count() + 1;
        let vals: Vec<u8> = input
            .split_whitespace()
            .flat_map(|l| l.chars())
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect();
        let len = &vals.len();
        Day11Solution(Array2::from_shape_vec((len / rows, rows), vals).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Day11Solution;
    use crate::utils::solution::Solution;

    #[test]
    fn test() {
        Day11Solution::test(
            "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
            1656,
            0,
        )
    }
}
