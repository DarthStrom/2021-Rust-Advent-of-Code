use itertools::Itertools;

use crate::input;

pub fn run() {
    let lines = input::get_lines("day11");

    let part1 = flashes(&lines, 100);

    println!("part1: {:?}", part1);

    let part2 = first_simultaneous(&lines);

    println!("part2: {:?}", part2);
}

fn flashes(lines: &[String], steps: u32) -> u32 {
    let mut result = 0;
    let mut cavern = get_cavern(lines);
    for _step in 1..=steps {
        let flashes = perform_step(&mut cavern);
        result += flashes;
    }
    result
}

fn first_simultaneous(lines: &[String]) -> u32 {
    let mut cavern = get_cavern(lines);
    let mut step = 0;
    while !simultaneous(&cavern) {
        let _ = perform_step(&mut cavern);
        step += 1;
    }
    step
}

fn simultaneous(cavern: &[Vec<u32>]) -> bool {
    cavern.iter().all(|row| row.iter().all(|&o| o == 0))
}

fn get_cavern(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn perform_step(cavern: &mut Vec<Vec<u32>>) -> u32 {
    for row in cavern.iter_mut() {
        for octopus in row.iter_mut() {
            *octopus += 1;
        }
    }
    let mut flashes = 0;
    let mut flashed: Vec<(usize, usize)> = vec![];
    for (y, row) in cavern.clone().iter().enumerate() {
        for (x, octopus) in row.iter().enumerate() {
            match octopus {
                1..=9 => (),
                _ => flashes = flash(cavern, &mut flashed, x, y, flashes),
            }
        }
    }
    for row in cavern.iter_mut() {
        for octopus in row.iter_mut() {
            if *octopus > 9 {
                *octopus = 0;
            }
        }
    }
    flashes
}

fn flash(
    cavern: &mut [Vec<u32>],
    flashed: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    flashes: u32,
) -> u32 {
    if flashed.contains(&(x, y)) {
        return flashes;
    }
    let mut flashes = flashes + 1;
    flashed.push((x, y));

    if x > 0 && y < cavern.len() - 1 {
        flashes = flash_neighbor(cavern, flashed, x - 1, y + 1, flashes)
    }
    if x > 0 {
        flashes = flash_neighbor(cavern, flashed, x - 1, y, flashes)
    }
    if x > 0 && y > 0 {
        flashes = flash_neighbor(cavern, flashed, x - 1, y - 1, flashes)
    }
    if y > 0 {
        flashes = flash_neighbor(cavern, flashed, x, y - 1, flashes)
    }
    if x < cavern[y].len() - 1 && y > 0 {
        flashes = flash_neighbor(cavern, flashed, x + 1, y - 1, flashes)
    }
    if x < cavern[y].len() - 1 {
        flashes = flash_neighbor(cavern, flashed, x + 1, y, flashes)
    }
    if x < cavern[y].len() - 1 && y < cavern.len() - 1 {
        flashes = flash_neighbor(cavern, flashed, x + 1, y + 1, flashes)
    }
    if y < cavern.len() - 1 {
        flashes = flash_neighbor(cavern, flashed, x, y + 1, flashes)
    }

    flashes
}

fn flash_neighbor(
    cavern: &mut [Vec<u32>],
    flashed: &mut Vec<(usize, usize)>,
    x: usize,
    y: usize,
    flashes: u32,
) -> u32 {
    cavern[y][x] += 1;
    if cavern[y][x] > 9 {
        return flash(cavern, flashed, x, y, flashes);
    }
    flashes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    #[test]
    fn small_example() {
        let lines = ["11111", "19991", "19191", "19991", "11111"].map(|s| s.to_string());

        assert_eq!(flashes(&lines, 0), 0);
        assert_eq!(flashes(&lines, 1), 9);
        assert_eq!(flashes(&lines, 2), 9);
    }

    #[test]
    fn example1() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(flashes(&lines, 0), 0);
        assert_eq!(flashes(&lines, 1), 0);
        assert_eq!(flashes(&lines, 2), 35);
        assert_eq!(flashes(&lines, 10), 204);
        assert_eq!(flashes(&lines, 100), 1656);
    }

    #[test]
    fn simultaneous_flashing() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(first_simultaneous(&lines), 195);
    }
}
