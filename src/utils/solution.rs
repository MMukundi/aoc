use std::fmt::Display;

pub trait Solution {
    type Answer:Display;
    fn num()->usize;
    fn units()->String;
    fn get()->std::io::Result<Self> where Self: Sized{ Ok(Self::solve(std::fs::read_to_string(format!("./res/day{}.txt",Self::num()).as_str())?)) }
    
    fn solve(input:String)->Self;

    fn display_solutions(&mut self)->(){
        let units = Self::units();
        println!("Day {}:", Self::num());
        println!("\tStar 1: {} {}", self.solve_first_star(),units);
        println!("\tStar 2: {} {}", self.solve_second_star(),units);
    }

    fn solve_first_star(&mut self)->Self::Answer;
    fn solve_second_star(&mut self)->Self::Answer;
}
pub trait Solvable{
    fn num(&self)->usize;
    fn solve(&mut self);
}
impl <S:Solution> Solvable for S{
    fn num(&self)->usize{
        Self::num()
    }
    fn solve(&mut self) {
        self.display_solutions()
    }
}