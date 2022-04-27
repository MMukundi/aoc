use crate::utils::solution::Solution;

fn count_increases<T: PartialOrd<T>>(values: &Vec<T>) -> usize {
    values.windows(2).filter(|pair| pair[1] > pair[0]).count()
}
pub struct Day1Solution {
    heights: Vec<u32>,
}
impl Solution for Day1Solution {
    type Answer = usize;

    fn num() -> usize {
        1
    }
    fn units() -> String {
        "increases".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        count_increases(&self.heights)
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        count_increases(
            &self
                .heights
                .windows(3)
                .map(|trip| trip.iter().sum())
                .collect::<Vec<u32>>(),
        )
    }
    fn solve(input: String) -> Day1Solution {
        Day1Solution {
            heights: input
                .split_whitespace()
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        }
    }
}
