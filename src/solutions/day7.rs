use crate::utils::solution::Solution;
pub struct Day7Solution(Vec<usize>,std::ops::Range<usize>);
impl Day7Solution{
    fn min_dist<F:Fn(usize,usize)->usize>(&self,metric:F)->usize{
        self.1.clone().into_iter().map(|dest|self.0.iter().map(|p|metric(*p,dest)).sum::<usize>()).min().unwrap_or_default()
    }
}
impl Solution for Day7Solution{
    type Answer=usize;

    fn num() -> usize { 7 }
    fn units() -> String { "units".to_string() }
    fn solve_first_star(&mut self) -> Self::Answer { 
        self.min_dist(|p,dest|p.abs_diff(dest))
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        self.min_dist(|p,dest|{
            let d= p.abs_diff(dest);
            d*(d+1)/2
        })
    }

    fn solve(input:String)->Self {
        let positions:Vec<usize> = input.split(",").map(|str|str.parse::<usize>().unwrap_or_default()).collect();
        let (min,max) = positions.iter().fold((usize::MAX,usize::MIN), |(min,max),pos|(min.min(*pos), max.max(*pos)));
        Day7Solution(input.split(",").map(|str|str.parse::<usize>().unwrap_or_default()).collect(),std::ops::Range{start:min,end:max})
    }
}

#[cfg(test)]
mod test {
    use crate::utils::solution::Solution;

    use super::Day7Solution;

    #[test]
    fn solve(){
        let mut solution = Day7Solution::solve("16,1,2,0,4,2,7,1,2,14".to_string());
        assert_eq!(solution.solve_first_star(),37);
        assert_eq!(solution.solve_second_star(),168);
    }
}
