use crate::utils::solution::Solution;
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ChunkType {
    Round,
    Square,
    Curly,
    Angle,
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ChunkChar {
    Open(ChunkType),
    Close(ChunkType),
}
impl TryFrom<char> for ChunkChar {
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(ChunkChar::Open(ChunkType::Round)),
            '[' => Ok(ChunkChar::Open(ChunkType::Square)),
            '{' => Ok(ChunkChar::Open(ChunkType::Curly)),
            '<' => Ok(ChunkChar::Open(ChunkType::Angle)),

            ')' => Ok(ChunkChar::Close(ChunkType::Round)),
            ']' => Ok(ChunkChar::Close(ChunkType::Square)),
            '}' => Ok(ChunkChar::Close(ChunkType::Curly)),
            '>' => Ok(ChunkChar::Close(ChunkType::Angle)),
            _ => Err(c),
        }
    }
    type Error = char;
}
pub enum SyntaxError {
    Incomplete(Vec<ChunkType>),
    Corrupted(ChunkType),
}
pub fn find_corruption<T: Iterator<Item = char>>(chars: T) -> Option<SyntaxError> {
    let mut open_chunks: Vec<ChunkType> = Vec::default();
    chars
        .map(|c| ChunkChar::try_from(c).unwrap())
        .find(|c| match c {
            ChunkChar::Open(chunk_type) => {
                open_chunks.push(*chunk_type);
                false
            }
            ChunkChar::Close(chunk_type) => match open_chunks.last() {
                Some(last_type) if *last_type == *chunk_type => {
                    open_chunks.pop();
                    false
                }
                _ => true,
            },
        })
        .map(|corrupt| match corrupt {
            ChunkChar::Close(c) | ChunkChar::Open(c) => c,
        })
        .map(|c| SyntaxError::Corrupted(c))
        .or_else(|| {
            if open_chunks.len() == 0 {
                None
            } else {
                Some(SyntaxError::Incomplete(open_chunks))
            }
        })
}
impl SyntaxError {
    fn score(self) -> usize {
        match self {
            SyntaxError::Corrupted(corrupt_type) => match corrupt_type {
                ChunkType::Round => 3,
                ChunkType::Square => 57,
                ChunkType::Curly => 1197,
                ChunkType::Angle => 25137,
            },
            SyntaxError::Incomplete(types) => types.iter().rev().fold(0, |score, t| {
                5 * score
                    + match t {
                        ChunkType::Round => 1,
                        ChunkType::Square => 2,
                        ChunkType::Curly => 3,
                        ChunkType::Angle => 4,
                    }
            }),
        }
    }
}
pub struct Day10Solution(Vec<String>);
impl Day10Solution {
    fn score<'a, F: 'a + FnMut(&SyntaxError) -> bool>(
        &'a self,
        predicate: F,
    ) -> impl Iterator<Item = usize> + 'a {
        self.0
            .iter()
            .map(|s| find_corruption(s.chars()))
            .flatten()
            .filter(predicate)
            .map(SyntaxError::score)
    }
}
impl Solution for Day10Solution {
    type Answer = usize;

    fn num() -> usize {
        10
    }
    fn units() -> String {
        "lines".to_string()
    }
    fn solve_first_star(&mut self) -> Self::Answer {
        self.score(|s| {
            if let SyntaxError::Corrupted(_) = s {
                true
            } else {
                false
            }
        })
        .sum()
    }

    fn solve_second_star(&mut self) -> Self::Answer {
        let mut scores: Vec<usize> = self
            .score(|s| {
                if let SyntaxError::Incomplete(_) = s {
                    true
                } else {
                    false
                }
            })
            .collect();
        scores.sort();
        scores[scores.len() / 2]
    }
    fn solve(input: String) -> Self {
        Day10Solution(input.lines().map(|s| s.to_owned()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::Day10Solution;
    use crate::utils::solution::Solution;

    #[test]
    fn test() {
        Day10Solution::test(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
            26397,
            288957,
        )
    }
}
