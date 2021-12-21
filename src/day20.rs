use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_contents("day16");

    println!("part1: {:?}", input);
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coords(i32, i32);

struct Image {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

fn get_algorithm_and_image(input: &str) -> (Vec<bool>, HashMap<Coords, bool>) {
    let mut lines_iter = input.lines();
    let algorithm = lines_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("not recognized"),
        })
        .collect_vec();
    let _blank = lines_iter.next();
    let image = lines_iter
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        Coords(x as i32, y as i32),
                        match c {
                            '.' => false,
                            '#' => true,
                            _ => panic!("not recognized"),
                        },
                    )
                })
                .collect::<HashMap<Coords, bool>>()
        })
        .reduce(|a, b| a.into_iter().chain(b).collect())
        .unwrap();
    (algorithm, image)
}

fn iterate(algorithm: &[bool], image: &HashMap<Coords, bool>) -> HashMap<Coords, bool> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn parsing() {
        let (algorithm, image) = get_algorithm_and_image(INPUT);

        assert_eq!(algorithm.len(), 512);
        assert_eq!(image.len(), 25);
    }

    // #[test]
    // fn iterating() {
    //     let (algorithm, image) = get_algorithm_and_image(INPUT);

    //     assert_eq!(iterate(&algorithm, &image).len(), 49);
    // }
}
