use crate::utils::solution::{Solvable, Solution};

mod day1;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use {day1::*,day4::*,day5::*,day6::*,day7::*,day8::*,day9::*};

pub fn get_solutions()->std::io::Result<Vec<Box<dyn Solvable>>>{
    Ok(vec![
        Box::new(Day1Solution::get()?),
        Box::new(Day4Solution::get()?),
        Box::new(Day5Solution::get()?),
        Box::new(Day6Solution::get()?),
        Box::new(Day7Solution::get()?),
        Box::new(Day8Solution::get()?),
        Box::new(Day9Solution::get()?)
    ])
}