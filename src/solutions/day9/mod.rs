use crate::utils::solution::Solution;
type Pair = (usize,usize);
pub struct HeightMap(Vec<u8>,Pair);
impl HeightMap{
    fn get(&self, (x,y): Pair) -> Option<&u8> {
        if x<self.1.0 &&y<self.1.1{
            Some(&self.0[x+y*self.1.0])
        } else {
            None
        }
    }
}
impl std::ops::Index<Pair> for HeightMap {
    type Output=u8;

    fn index(&self, pair: Pair) -> &Self::Output {
        self.get(pair).unwrap_or(&9)
    }
}
pub struct Day9Solution(HeightMap);
impl Day9Solution{}
impl Solution for Day9Solution{
    type Answer=usize;

    fn num()->usize { 9 }
    fn units() ->String { todo!() }
    fn solve_first_star(&mut self)->Self::Answer { 0 }

    fn solve_second_star(&mut self)->Self::Answer { 0 }

    fn solve(input:String)->Self {
        let y_size = input.matches("\n").count();
        let heights:Vec<u8> = input.lines().map(|l|l.chars()).flatten().map(|c|c.to_digit(10).unwrap_or(9)).map(|n|n as u8).collect();
        let x_size = heights.len()/y_size;
        Day9Solution(HeightMap(heights,(x_size,y_size)))
    }
}