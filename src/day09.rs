use itertools::Itertools;
use std::collections::HashSet;

use crate::input;

pub fn run() {
    let heightmap = input::get_lines("day09");

    let part1 = risk_level(&heightmap);

    println!("part1: {:?}", part1);

    let part2 = basin_product(&heightmap);

    println!("part2: {:?}", part2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Location {
    x: usize,
    y: usize,
    height: u32,
}

fn get_array(heightmap: &[String]) -> Vec<Vec<u32>> {
    heightmap
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

fn get_low_points(a: &[Vec<u32>]) -> Vec<Location> {
    let mut result = vec![];
    for y in 0..a.len() {
        let row = a.get(y).unwrap();
        for x in 0..row.len() {
            let current = Location {
                x,
                y,
                height: a[y][x],
            };
            let neighbors = get_neighbors(a, &current);
            if neighbors.iter().all(|n| current.height < n.height) {
                result.push(current)
            }
        }
    }
    result
}

fn get_neighbors(a: &[Vec<u32>], current: &Location) -> Vec<Location> {
    let mut result = vec![];

    if let Some(row) = if current.y > 0 {
        a.get(current.y - 1)
    } else {
        None
    } {
        result.push(Location {
            x: current.x,
            y: current.y - 1,
            height: row[current.x],
        });
    }
    if let Some(row) = a.get(current.y + 1) {
        result.push(Location {
            x: current.x,
            y: current.y + 1,
            height: row[current.x],
        });
    }
    let left = if current.x > 0 {
        a[current.y].get(current.x - 1)
    } else {
        None
    };
    if current.x > 0 && left.is_some() {
        result.push(Location {
            x: current.x - 1,
            y: current.y,
            height: *a[current.y].get(current.x - 1).unwrap(),
        })
    }
    let right = a[current.y].get(current.x + 1);
    if right.is_some() {
        result.push(Location {
            x: current.x + 1,
            y: current.y,
            height: *a[current.y].get(current.x + 1).unwrap(),
        })
    }
    result
}

fn risk_level(heightmap: &[String]) -> u32 {
    let a = get_array(heightmap);

    get_low_points(&a).iter().map(|p| p.height + 1).sum()
}

fn basin_product(heightmap: &[String]) -> u32 {
    let a = get_array(heightmap);
    let low_points = get_low_points(&a);

    low_points
        .iter()
        .map(|p| basin_members(&a, p).len() as u32)
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn basin_members(a: &[Vec<u32>], point: &Location) -> HashSet<Location> {
    let mut result = HashSet::new();
    result.insert(*point);
    get_neighbors(a, point)
        .iter()
        .filter(|n| n.height > point.height && n.height < 9)
        .map(|l| basin_members(a, l))
        .fold(result, |acc, cur| {
            acc.union(&cur).copied().collect::<HashSet<Location>>()
        })
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

    #[test]
    fn example2() {
        assert_eq!(basin_product(&INPUT.map(|s| s.to_string())), 1134);
    }
}
