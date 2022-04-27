use solutions::get_solutions;

pub mod utils;
mod solutions;
enum Mode{
    All,
    First,
    Last,
    Only(usize)
}
impl Default for Mode{fn default() -> Self { Mode::All }}
fn main() ->std::io::Result<()>{
    let input = std::env::args().nth(1);
    let chr = match &input {
        Some(s)=>s.chars().nth(0).unwrap_or('a'),
        None=>'a'
    };
    let mut solutions = get_solutions()?;
    let mode = match chr {
        '0'..='9'=>input.map(|s|s.parse::<usize>().map(Mode::Only).unwrap_or_default()).unwrap_or_default(),
        'l'=>Mode::Last,
        'a'=>Mode::All,
        'f'=>Mode::First,
        _=>Mode::All
    };
    match mode {
        Mode::All=>{
            solutions.iter_mut().for_each(|s|s.solve());
        },
        Mode::Last=>{
            solutions.iter_mut().last().map(|s|s.solve());
        },
        Mode::First=>{
            solutions.iter_mut().nth(0).map(|s|s.solve());
        },
        Mode::Only(n)=>{
            solutions.iter_mut().filter(|s|s.num()==n).for_each(|s|s.solve());
        },
    };
    Ok(())
}
