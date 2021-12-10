use itertools::Itertools;

use crate::input;

pub fn run() {
    let lines = input::get_lines("day10");

    let part1 = error_score(&lines);

    println!("part1: {:?}", part1);

    let part2 = autocorrect_score(&lines);

    println!("part2: {:?}", part2);
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
    Incomplete(Vec<char>),
}

impl SyntaxError {
    fn score(&self) -> usize {
        match self {
            Self::Incorrect(Chunk::Paren) => 3,
            Self::Incorrect(Chunk::Bracket) => 57,
            Self::Incorrect(Chunk::Brace) => 1197,
            Self::Incorrect(Chunk::Angle) => 25137,
            Self::Incomplete(unresolved) => unresolved.iter().rev().fold(0, |acc, cur| {
                5 * acc
                    + match cur {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0,
                    }
            }),
        }
    }
}

fn find_errors(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .map(|line| find_error(line))
        .filter(|e| matches!(e.as_ref().err(), Some(SyntaxError::Incorrect(_))))
        .map(|e| e.err().unwrap().score())
        .collect_vec()
}

fn error_score(lines: &[String]) -> usize {
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
        Err(SyntaxError::Incomplete(unresolved))
    }
}

fn find_autocorrects(lines: &[String]) -> Vec<usize> {
    lines
        .iter()
        .map(|line| find_error(line))
        .filter(|e| matches!(e.as_ref().err(), Some(SyntaxError::Incomplete(_))))
        .map(|e| e.err().unwrap().score())
        .collect_vec()
}

fn autocorrect_score(lines: &[String]) -> usize {
    let autocorrects = find_autocorrects(lines).into_iter().sorted().collect_vec();
    autocorrects[autocorrects.len() / 2]
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
        assert_eq!(error_score(&lines), 26397);
    }

    #[test]
    fn example2() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(
            find_autocorrects(&lines),
            [288957, 5566, 1480781, 995444, 294]
        );
        assert_eq!(autocorrect_score(&lines), 288957);
    }
}
