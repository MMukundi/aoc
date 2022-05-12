use std::collections::HashSet;
use std::hash::Hash;
pub trait Adjacency {
    fn adjacenies(&self) -> Vec<Self>
    where
        Self: Sized;
}

pub struct FloodfillIter<T: Adjacency + Eq + Hash + Clone, Pred: Fn(&T) -> bool> {
    to_visit: std::collections::HashSet<T>,
    visited: std::collections::HashSet<T>,
    pred: Pred,
}
impl<T: Adjacency + Eq + Hash + Clone, Pred: Fn(&T) -> bool> Iterator for FloodfillIter<T, Pred> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let next_point = self.to_visit.iter().cloned().next();
        next_point.map(|p| {
            let adj = p.adjacenies();
            self.to_visit.remove(&p);
            self.visited.insert(p.clone());
            self.to_visit.extend(
                adj.into_iter()
                    .filter(|a| !self.visited.contains(a))
                    .filter(&self.pred),
            );
            // self.to_visit.extend(adj.into_iter().filter(&self.pred));
            p
        })
    }
}

pub trait Floodfill<'a, T: Adjacency + Eq + Hash + Clone> {
    fn floodfill<Pred: Fn(&T) -> bool>(self, pred: Pred) -> FloodfillIter<T, Pred>;
}
impl<'a, T: Adjacency + Eq + Hash + Clone> Floodfill<'a, T> for T {
    fn floodfill<Pred: Fn(&T) -> bool>(self, pred: Pred) -> FloodfillIter<T, Pred> {
        FloodfillIter {
            visited: Default::default(),
            to_visit: HashSet::from([self]),
            pred,
        }
    }
}
