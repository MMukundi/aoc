use std::collections::HashMap;

use crate::utils::solution::Solution;

use self::entry::{Entry, Segments};

mod entry;

#[derive(Debug)]
pub struct Day8Solution(Vec<Entry>);
impl Day8Solution {}
impl Solution for Day8Solution {
    type Answer = usize;

    fn num() -> usize {
        8
    }
    fn units() -> String {
        "".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        self.0
            .iter()
            .map(|e| {
                e.outputs
                    .iter()
                    .filter(|o| match o.count_active() {
                        2 | 3 | 4 | 7 => true,
                        _ => false,
                    })
                    .count()
            })
            .sum()
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        let actuals = (0..10)
            .map(Entry::segments_for)
            .collect::<Vec<Segments>>();
        let digit_map = actuals
            .iter()
            .enumerate()
            .map(|(i, s)| (*s, i))
            .collect::<HashMap<Segments, usize>>();
        self.0
            .iter_mut()
            .map(|e| {
                e.signal_patterns.clone().iter().for_each(|found| {
                    let count = found.count_active();
                    match count {
                        2 => e.restrict_to(&actuals[1], found),
                        3 => e.restrict_to(&actuals[7], found),
                        4 => e.restrict_to(&actuals[4], found),
                        5 => {
                            e.restrict_to(&(actuals[2] & actuals[3] & actuals[5]), found);
                        }
                        6 => {
                            e.restrict_to(&(actuals[0] & actuals[6] & actuals[9]), found);
                        }
                        7 => e.restrict_to(&actuals[8], found),
                        _ => (),
                    };
                });
                let map = e
                    .could_be
                    .iter()
                    .enumerate()
                    .map(|(i, s)| (*s, Segments::from(1 << i)))
                    .collect::<HashMap<Segments, Segments>>();
                let output = e
                    .outputs
                    .iter()
                    .map(|out| {
                        let digit_segment = out.iter().map(|s| map[&s]).reduce(|a, b| a | b);
                        digit_map[&digit_segment.unwrap()]
                    })
                    .fold(0, |num, digit| 10 * num + digit);
                output
            })
            .sum()
    }
    fn solve(input: String) -> Self {
        Day8Solution(
            input
                .lines()
                .map(|line| line.parse::<Entry>().unwrap())
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::utils::solution::Solution;

    use super::Day8Solution;

    #[test]
    fn simp() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let mut solution = Day8Solution::solve(input.to_string());
        assert_eq!(solution.solve_second_star(), 5353);
    }

    #[test]
    fn solve() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let mut solution = Day8Solution::solve(input.to_string());
        assert_eq!(solution.solve_first_star(), 26);
        assert_eq!(solution.solve_second_star(), 61229);
    }
}
