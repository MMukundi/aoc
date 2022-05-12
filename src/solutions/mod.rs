use crate::utils::solution::{Solution, SolutionError, Solvable};

mod day1;
mod day10;
mod day11;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use {day1::*, day10::*, day11::*, day4::*, day5::*, day6::*, day7::*, day8::*, day9::*};

pub fn get_solutions() -> Result<Vec<Box<dyn Solvable>>, SolutionError> {
    Ok(vec![
        Box::new(Day1Solution::get()?),
        Box::new(Day4Solution::get()?),
        Box::new(Day5Solution::get()?),
        Box::new(Day6Solution::get()?),
        Box::new(Day7Solution::get()?),
        Box::new(Day8Solution::get()?),
        Box::new(Day9Solution::get()?),
        Box::new(Day10Solution::get()?),
        Box::new(Day11Solution::get()?),
    ])
}
