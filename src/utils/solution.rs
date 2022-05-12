use std::{
    error::Error,
    fmt::{Debug, Display},
};
#[derive(Debug)]
pub enum SolutionError {
    NoInputFile,
}
impl Display for SolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &SolutionError::NoInputFile => write!(f, "NoInputFile"),
        }
    }
}
impl Error for SolutionError {}
pub trait Solution {
    type Answer: Eq + Debug + Display;
    fn num() -> usize;
    fn units() -> String;
    fn get() -> Result<Self, SolutionError>
    where
        Self: Sized,
    {
        std::fs::read_to_string(format!("./res/day{}.txt", Self::num()).as_str())
            .map_err(|_| SolutionError::NoInputFile)
            .map(Self::solve)
    }

    fn solve(input: String) -> Self;

    fn display_solutions(&mut self) {
        let units = Self::units();
        println!("Day {}:", Self::num());
        println!("\tStar 1: {} {}", self.solve_first_star(), units);
        println!("\tStar 2: {} {}", self.solve_second_star(), units);
    }

    fn solve_first_star(&mut self) -> Self::Answer;
    fn solve_second_star(&mut self) -> Self::Answer;
    fn test(input: impl Into<String>, first: Self::Answer, second: Self::Answer)
    where
        Self: Sized,
    {
        let mut solution = Self::solve(input.into());
        assert_eq!(solution.solve_first_star(), first);
        assert_eq!(solution.solve_second_star(), second);
    }
}
pub trait Solvable {
    fn num(&self) -> usize;
    fn solve(&mut self);
}
impl<S: Solution> Solvable for S {
    fn num(&self) -> usize {
        Self::num()
    }
    fn solve(&mut self) {
        self.display_solutions()
    }
}
