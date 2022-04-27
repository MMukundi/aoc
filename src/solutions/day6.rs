use crate::utils::solution::Solution;

pub struct Day6Solution {
    pools: [usize; 9],
    birthing_pool: usize,
    day: usize,
}
impl Day6Solution {
    fn sim_day(&mut self) -> () {
        let new_fish = self.pools[self.birthing_pool];
        self.pools[(self.birthing_pool + 7) % 9] += new_fish;
        self.birthing_pool = (self.birthing_pool + 1) % 9;
        self.day += 1;
    }
    fn fish_count(&self) -> usize {
        self.pools.iter().sum()
    }
    fn count_after(&mut self, day: usize) -> usize {
        let additional_days = day - self.day;
        for _ in 0..additional_days {
            self.sim_day();
        }
        self.fish_count()
    }
}
impl Solution for Day6Solution {
    type Answer = usize;

    fn num() -> usize {
        6
    }
    fn units() -> String {
        "laternfish".to_string()
    }

    fn solve_first_star(&mut self) -> Self::Answer {
        self.count_after(80)
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        self.count_after(256)
    }
    fn solve(input: String) -> Day6Solution {
        let pools = input
            .split(",")
            .map(|str| str.parse::<usize>().unwrap_or_default())
            .fold([0; 9], |mut all_fish, next_fish| {
                all_fish[next_fish] += 1;
                all_fish
            });
        Day6Solution {
            pools,
            birthing_pool: 0,
            day: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::solution::Solution;

    use super::Day6Solution;

    #[test]
    fn solve() {
        assert_eq!(
            Day6Solution::solve("3,4,3,1,2".to_string()).solve_first_star(),
            5934
        );
        assert_eq!(
            Day6Solution::solve("3,4,3,1,2".to_string()).solve_second_star(),
            26984457539
        );
    }
}
