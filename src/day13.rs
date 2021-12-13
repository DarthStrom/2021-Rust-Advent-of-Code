use std::collections::HashSet;

use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_lines("day13");

    let part1 = fold(&input, 1);

    println!("part1: {:?}", part1);

    let part2 = fold_all(&input);

    for line in part2 {
        println!("{}", line);
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Left(usize),
    Up(usize),
}

fn fold_all(input: &[String]) -> Vec<String> {
    let (mut coordinates, instructions) = parse_input(input);

    for instruction in instructions {
        coordinates = do_instruction(instruction, &coordinates);
    }

    get_grid(&coordinates)
        .iter()
        .map(|row| {
            row.iter()
                .map(|val| match val {
                    true => '#',
                    false => '.',
                })
                .collect::<String>()
        })
        .collect_vec()
}

fn fold(input: &[String], steps: usize) -> usize {
    let (mut coordinates, instructions) = parse_input(input);

    for step in 1..=steps {
        coordinates = do_instruction(instructions[step - 1], &coordinates);
    }

    coordinates.len()
}

fn parse_input(input: &[String]) -> (HashSet<(usize, usize)>, Vec<Instruction>) {
    (
        input
            .iter()
            .take_while(|s| !s.is_empty())
            .map(|s| {
                let mut split = s.split(',');
                let x = split.next().unwrap().parse::<usize>().unwrap();
                let y = split.next().unwrap().parse::<usize>().unwrap();
                (x, y)
            })
            .collect(),
        input
            .iter()
            .skip_while(|s| !s.is_empty())
            .skip(1)
            .map(|s| {
                let mut split = s.split('=');
                match split.next() {
                    Some("fold along y") => Instruction::Up(split.next().unwrap().parse().unwrap()),
                    Some("fold along x") => {
                        Instruction::Left(split.next().unwrap().parse().unwrap())
                    }
                    _ => panic!("unknown instruction"),
                }
            })
            .collect_vec(),
    )
}

fn do_instruction(
    instruction: Instruction,
    coordinates: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::<(usize, usize)>::new();
    match instruction {
        Instruction::Up(line) => {
            coordinates
                .iter()
                .filter(|(_x, y)| y > &line)
                .for_each(|(x, y)| {
                    result.insert((*x, line - (y - line)));
                });
            coordinates
                .iter()
                .filter(|(_x, y)| y < &line)
                .for_each(|c| {
                    result.insert(*c);
                });
            result
        }
        Instruction::Left(line) => {
            coordinates
                .iter()
                .filter(|(x, _y)| x > &line)
                .for_each(|(x, y)| {
                    result.insert((line - (x - line), *y));
                });
            coordinates
                .iter()
                .filter(|(x, _y)| x < &line)
                .for_each(|c| {
                    result.insert(*c);
                });
            result
        }
    }
}

fn get_grid(coordinates: &HashSet<(usize, usize)>) -> Vec<Vec<bool>> {
    let max_x = coordinates.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = coordinates.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; max_x + 1]; max_y + 1];
    for &(x, y) in coordinates {
        grid[y][x] = true
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 21] = [
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn example1() {
        let input = INPUT.map(|s| s.to_string());

        assert_eq!(fold(&input, 1), 17);
        assert_eq!(fold(&input, 2), 16);
    }

    #[test]
    fn finish_folding() {
        let input = INPUT.map(|s| s.to_string());

        assert_eq!(
            fold_all(&input),
            vec![
                "#####".to_string(),
                "#...#".to_string(),
                "#...#".to_string(),
                "#...#".to_string(),
                "#####".to_string(),
            ]
        );
    }
}
