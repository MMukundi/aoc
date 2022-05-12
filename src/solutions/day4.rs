use std::str::FromStr;

use crate::utils::{solution::Solution,iter::IterAddons};

#[derive(Debug)]
struct Board(Vec<usize>, Vec<bool>, Option<(usize, usize)>);

impl Board {
    fn update(&mut self, ball: &usize, i: usize) {
        if self.2.is_none() {
            if let Some(i)=self.0
                .iter()
                .position(|b| b == ball)
                {
                    self.1[i] = true
                }
            self.2 = if self.has_won() {
                Some((i, *ball))
            } else {
                None
            }
        }
    }
    fn all_true<I: Iterator<Item = usize>>(&self, iter: &mut I) -> bool {
        iter.all(|i| self.1[i])
    }
    fn has_won(&self) -> bool {
        (0usize..25)
            .to_chunks(5)
            .any(|i| self.all_true(&mut i.into_iter()))
            || (0usize..5)
                .map(|c| (0usize..25).skip(c).step_by(5))
                .any(|mut i| self.all_true(&mut i))
    }
    fn score(&self) -> usize {
        self.0
            .iter()
            .zip(self.1.iter())
            .filter(|(_, b)| !**b)
            .map(|(n, _)| n)
            .sum()
    }
    fn win_score(&self) -> Option<usize> {
        self.2.map(|(_, win_ball)| self.score() * win_ball)
    }
}
#[cfg(test)]
mod tests {
    use super::Board;
    #[test]
    fn won() {
        let mut b = Board(vec![], vec![false; 25], None);
        (0..25).step_by(5).for_each(|i| b.1[i] = true);
        assert!(b.has_won())
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Ok(Board(nums, vec![false; 25], None))
    }
}

pub struct Day4Solution {
    sorted_boards: Vec<Board>,
}
impl Solution for Day4Solution {
    type Answer = usize;

    fn num() -> usize {
        4
    }
    fn units() -> String {
        "points".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        self.sorted_boards[0].win_score().unwrap_or_default()
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        self.sorted_boards
            .last()
            .map(|b| b.win_score())
            .unwrap_or_default()
            .unwrap_or_default()
    }
    fn solve(input: String) -> Day4Solution {
        let mut lines = input.split("\n\n");
        let balls = lines
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut boards: Vec<Board> = lines.map(|line| line.parse::<Board>().unwrap()).collect();
        for (round, ball) in balls.iter().enumerate() {
            for board in boards.iter_mut() {
                board.update(ball, round);
            }
        }
        boards.sort_by_cached_key(|b| b.2.map(|w| w.0));

        Day4Solution {
            sorted_boards: boards,
        }
    }
}
