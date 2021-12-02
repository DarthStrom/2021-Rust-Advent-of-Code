use core::panic;

use crate::input;

pub fn run() {
    let contents = input::get_lines("day02");
    let result = navigate(&contents);

    println!("part1: {:?}", result.0 * result.1);

    let result2 = corrected(&contents);

    println!("part2: {:?}", result2.0 * result2.1);
}

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn corrected(input: &[String]) -> (i32, i32) {
    let mut result = (0, 0);
    let mut aim = 0;
    for line in input {
        match parse_instruction(line) {
            Instruction::Down(d) => aim += d,
            Instruction::Forward(f) => {
                result.0 += f;
                result.1 += aim * f;
            }
            Instruction::Up(u) => aim -= u,
        }
    }
    result
}

fn navigate(input: &[String]) -> (i32, i32) {
    let mut result = (0, 0);
    for line in input {
        match parse_instruction(line) {
            Instruction::Down(d) => result.1 += d,
            Instruction::Forward(f) => result.0 += f,
            Instruction::Up(u) => result.1 -= u,
        }
    }
    result
}

fn parse_instruction(line: &str) -> Instruction {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    let val = parts[1].parse::<i32>().unwrap();
    match parts[0] {
        "forward" => Instruction::Forward(val),
        "down" => Instruction::Down(val),
        "up" => Instruction::Up(val),
        _ => panic!("unknown instruction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn example1() {
        assert_eq!(navigate(&input.map(|x| x.to_string())), (15, 10));
    }

    #[test]
    fn example2() {
        assert_eq!(corrected(&input.map(|x| x.to_string())), (15, 60))
    }
}
