use itertools::Itertools;

use crate::input;

pub fn run() {
    let heightmap = input::get_lines("day09");

    let part1 = risk_level(&heightmap);

    println!("part1: {:?}", part1);
}

fn risk_level(heightmap: &[String]) -> u32 {
    let a = heightmap
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut result = 0;

    for y in 0..a.len() {
        let row = a.get(y).unwrap();
        for x in 0..row.len() {
            let current = a[y][x];
            let above = if let Some(row) = if y > 0 { a.get(y - 1) } else { None } {
                row[x]
            } else {
                9
            };
            let below = if let Some(row) = a.get(y + 1) {
                row[x]
            } else {
                9
            };
            let left = if x > 0 {
                *a[y].get(x - 1).unwrap_or(&9)
            } else {
                9
            };
            let right = *a[y].get(x + 1).unwrap_or(&9);
            if current < above && current < below && current < left && current < right {
                result += current + 1
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 5] = [
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    #[test]
    fn example1() {
        assert_eq!(risk_level(&INPUT.map(|s| s.to_string())), 15);
    }
}
