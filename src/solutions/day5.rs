use std::str::FromStr;

use crate::utils::{point::{Point}, solution::Solution};

#[derive(Debug)]
struct Line(Point,Point);

impl Line{
    fn iter(&self) -> LineIter {
        let step = (self.1-self.0.clone()).to_step();
        LineIter{curr:self.0,end:self.1,step}
    }
}

struct LineIter {curr:Point,step:Point,end:Point}
impl Iterator for LineIter {
    type Item=Point;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr;
        let dot_product = (self.end-curr).dot(&self.step);        
        if dot_product >= 0{
            self.curr=self.curr+self.step;
            Some(curr)
        }else{
            None
        }
    }
}
impl  FromStr for Line {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ").map(|point_string|point_string.parse::<Point>().unwrap());
        Ok(Line(points.next().unwrap(),points.next().unwrap()))
    }
}

pub struct Day5Solution{lines:Vec<Line>, board_size:usize, board_bounds:Point}
impl Day5Solution{
    fn get_map(&self)->Vec<usize>{
        vec![0;self.board_size]
    }
    fn index(&self,x:i32,y:i32)->Option<usize>{
        (x+y*self.board_bounds.x).try_into().into_iter().next()
    }
    fn count_multiple_overlaps<'a>(&self,lines:impl Iterator<Item=&'a Line>)->usize{
        let mut map = self.get_map();
        for line in lines  {
            line.iter().for_each(|Point{x,y}|self.index(x,y).into_iter().for_each(|i|map[i]+=1));
        }
        map.iter().filter(|x|**x>=2).count()
    }
}
impl Solution for Day5Solution{
    type Answer=usize;

    fn num()->usize { 5 }
    fn units() ->String { "points".to_string() }
    fn solve_first_star(&mut self)->Self::Answer {   
        let horiz_or_vert = self.lines.iter().filter(|l|l.0.x==l.1.x||l.0.y==l.1.y);
        self.count_multiple_overlaps(horiz_or_vert)
    }

    fn solve_second_star(&mut self)->Self::Answer {  
        self.count_multiple_overlaps(self.lines.iter())
    }
    fn solve(input:String)->Day5Solution{
        let lines = input.lines().map(|line|line.parse::<Line>().unwrap()).collect::<Vec<Line>>();
    
        let board_bounds= lines.iter().fold(Point{x:0i32,y:0},|p,l|Point{
            x:std::cmp::max(std::cmp::max(p.x,l.0.x),l.1.x),
            y:std::cmp::max(std::cmp::max(p.y,l.0.y),l.1.y)
        })+Point::same(1);
    
        Day5Solution{board_size:(board_bounds.x*board_bounds.y).try_into().unwrap_or(0), lines ,board_bounds}
    }
}
