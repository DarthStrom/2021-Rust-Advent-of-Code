use core::panic;

use crate::input;

pub fn run() {
    let contents = input::get_lines("day02");
    let result = navigate(&contents);

    println!("part1: {:?}", result.0 * result.1);
}

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
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

    #[test]
    fn example1() {
        let input = [
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        assert_eq!(navigate(&input), (15, 10));
    }
}
