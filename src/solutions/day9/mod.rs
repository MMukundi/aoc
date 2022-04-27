use std::collections::HashSet;

use crate::utils::iter;
use crate::utils::point::Point;
use crate::utils::solution::Solution;

pub struct HeightMap(Vec<i8>, Point);
pub struct Basin<'a> {
    map: &'a HeightMap,
    to_check: HashSet<Point>,
    seen: HashSet<Point>,
}
impl Iterator for Basin<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.to_check.iter().next().cloned().map(|p| {
            let news = vec![
                Point { x: p.x + 1, y: p.y },
                Point { x: p.x - 1, y: p.y },
                Point { x: p.x, y: p.y + 1 },
                Point { x: p.x, y: p.y - 1 },
            ];
            self.to_check.extend(news.into_iter().filter(|p| {
                p.in_range(&Point::same(0), &self.map.1)
                    && !self.seen.contains(p)
                    && (self.map[(p.x, p.y)] != 9)
            }));
            self.to_check.remove(&p);
            self.seen.insert(p);
            p
        })
    }
}
impl HeightMap {
    fn get(&self, p: Point) -> Option<&i8> {
        if p.in_range(&Point::same(0), &self.1) {
            Some(&self.0[(p.x + p.y * self.1.x) as usize])
        } else {
            None
        }
    }
    fn basin(&self, point: Point) -> Basin {
        let mut to_check = HashSet::new();
        to_check.insert(point);
        Basin {
            map: self,
            to_check,
            seen: HashSet::new(),
        }
    }
    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.1.x)
            .map(|x| (0..self.1.y).map(move |y| (x, y)))
            .flatten()
    }
    fn low_points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.points().filter(|(x, y)| {
            let x = *x;
            let y = *y;
            let h = self[(x, y)];
            let is_low = (self[(x - 1, y)] > h)
                && (self[(x + 1, y)] > h)
                && (self[(x, y - 1)] > h)
                && (self[(x, y + 1)] > h);
            is_low
        })
    }
}
impl std::ops::Index<(i32, i32)> for HeightMap {
    type Output = i8;

    fn index(&self, pair: (i32, i32)) -> &Self::Output {
        self.get(Point {
            x: pair.0,
            y: pair.1,
        })
        .unwrap_or(&9)
    }
}
pub struct Day9Solution {
    map: HeightMap,
}
impl Day9Solution {}
impl Solution for Day9Solution {
    type Answer = usize;

    fn num() -> usize {
        9
    }
    fn units() -> String {
        "units".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        self.map
            .low_points()
            .map(|p| (self.map[p] + 1) as usize)
            .sum()
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        let mut counts = self
            .map
            .low_points()
            .map(|(x, y)| self.map.basin(Point { x, y }).count())
            .collect::<Vec<usize>>();
        counts.sort();
        counts.reverse();
        counts.into_iter().take(3).product()
    }

    fn solve(input: String) -> Self {
        let y_size = input.matches("\n").count() + 1;
        let heights: Vec<i8> = input
            .lines()
            .map(|l| l.chars())
            .flatten()
            .map(|c| c.to_digit(10).unwrap_or(9))
            .map(|n| n as i8)
            .collect();
        let x_size = heights.len() / y_size;
        Day9Solution {
            map: HeightMap(
                heights,
                Point {
                    x: x_size as i32,
                    y: y_size as i32,
                },
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day9Solution;
    use crate::utils::solution::Solution;

    #[test]
    fn test() {
        let val = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .to_string();
        let mut solution = Day9Solution::solve(val);
        assert_eq!(solution.solve_first_star(), 15);
        assert_eq!(solution.solve_second_star(), 1134);
    }
}
