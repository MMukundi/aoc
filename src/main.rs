use solutions::get_solutions;

mod solutions;
pub mod utils;
enum Mode {
    All,
    First,
    Last,
    Only(usize),
}
impl Default for Mode {
    fn default() -> Self {
        Mode::All
    }
}
fn main() {
    match get_solutions() {
        Ok(mut solutions) => {
            let input = std::env::args().nth(1);
            let chr = match &input {
                Some(s) => s.chars().next().unwrap_or('a'),
                None => 'a',
            };

            let mode = match chr {
                '0'..='9' => input
                    .map(|s| s.parse::<usize>().map(Mode::Only).unwrap_or_default())
                    .unwrap_or_default(),
                'l' => Mode::Last,
                'a' => Mode::All,
                'f' => Mode::First,
                _ => Mode::All,
            };
            match mode {
                Mode::All => {
                    solutions.iter_mut().for_each(|s| s.solve());
                }
                Mode::Last => {
                    if let Some(s) = solutions.iter_mut().last() { s.solve() }
                }
                Mode::First => {
                    if let Some(s) = solutions.iter_mut().nth(0) { s.solve() }
                }
                Mode::Only(n) => {
                    solutions
                        .iter_mut()
                        .filter(|s| s.num() == n)
                        .for_each(|s| s.solve());
                }
            };
        }
        Err(e) => println!("Error: {}", e),
    }
}
