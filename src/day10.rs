use itertools::Itertools;

use crate::input;

pub fn run() {
    let lines = input::get_lines("day10");

    let part1 = total_score(&lines);

    println!("part1: {:?}", part1);
}

#[derive(Debug, PartialEq, Eq)]
enum Chunk {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug, PartialEq, Eq)]
enum SyntaxError {
    Incorrect(Chunk),
    Incomplete,
}

impl SyntaxError {
    fn score(&self) -> u32 {
        match *self {
            Self::Incorrect(Chunk::Paren) => 3,
            Self::Incorrect(Chunk::Bracket) => 57,
            Self::Incorrect(Chunk::Brace) => 1197,
            Self::Incorrect(Chunk::Angle) => 25137,
            _ => 0,
        }
    }
}

fn find_errors(lines: &[String]) -> Vec<u32> {
    lines
        .iter()
        .map(|line| find_error(line))
        .filter(|e| matches!(e.as_ref().err(), Some(SyntaxError::Incorrect(_))))
        .map(|e| e.err().unwrap().score())
        .collect_vec()
}

fn total_score(lines: &[String]) -> u32 {
    find_errors(lines).iter().sum()
}

fn find_error(line: &str) -> Result<(), SyntaxError> {
    let mut unresolved = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => unresolved.push(c),
            ')' => {
                if unresolved.pop() != Some('(') {
                    return Err(SyntaxError::Incorrect(Chunk::Paren));
                }
            }
            ']' => {
                if unresolved.pop() != Some('[') {
                    return Err(SyntaxError::Incorrect(Chunk::Bracket));
                }
            }
            '}' => {
                if unresolved.pop() != Some('{') {
                    return Err(SyntaxError::Incorrect(Chunk::Brace));
                }
            }
            '>' => {
                if unresolved.pop() != Some('<') {
                    return Err(SyntaxError::Incorrect(Chunk::Angle));
                }
            }
            _ => panic!("unrecognized symbol"),
        }
    }
    if unresolved.is_empty() {
        Ok(())
    } else {
        Err(SyntaxError::Incomplete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn example1() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(find_errors(&lines), [1197, 3, 57, 3, 25137]);
        assert_eq!(total_score(&lines), 26397);
    }
}
