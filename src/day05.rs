use std::{collections::HashMap, str::Split};

use crate::input;

pub fn run() {
    let contents = input::get_lines("day05");

    let part1 = h_v_overlaps(&contents);

    println!("part1: {:?}", part1);

    let part2 = h_v_d_overlaps(&contents);

    println!("part2: {:?}", part2);
}

fn parse(lines: &[String]) -> Vec<((u32, u32), (u32, u32))> {
    lines
        .iter()
        .map(|line| {
            let mut coordinates = line.split(" -> ");
            let mut coordinate1 = coordinates.next().unwrap().split(',');
            let x1 = parse_coordinate(&mut coordinate1);
            let y1 = parse_coordinate(&mut coordinate1);
            let mut coordinate2 = coordinates.next().unwrap().split(',');
            let x2 = parse_coordinate(&mut coordinate2);
            let y2 = parse_coordinate(&mut coordinate2);
            ((x1, y1), (x2, y2))
        })
        .collect::<Vec<_>>()
}

fn parse_coordinate(coordinate: &mut Split<char>) -> u32 {
    coordinate.next().unwrap().parse::<u32>().unwrap()
}

fn h_v_overlaps(lines: &[String]) -> usize {
    let coordinates = parse(lines);
    let mut all_coordinates = HashMap::new();

    coordinates.iter().for_each(|((x1, y1), (x2, y2))| {
        if x1 == x2 {
            let range = if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 };
            for y in range {
                let entry = all_coordinates.entry((*x1, y)).or_insert(0);
                *entry += 1;
            }
        }
        if y1 == y2 {
            let range = if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 };
            for x in range {
                let entry = all_coordinates.entry((x, *y1)).or_insert(0);
                *entry += 1;
            }
        }
    });

    all_coordinates.into_iter().filter(|e| e.1 > 1).count()
}

fn h_v_d_overlaps(lines: &[String]) -> usize {
    let coordinates = parse(lines);
    let mut all_coordinates = HashMap::new();

    coordinates.iter().for_each(|((x1, y1), (x2, y2))| {
        if x1 == x2 {
            for y in if y1 < y2 { *y1..=*y2 } else { *y2..=*y1 } {
                let entry = all_coordinates.entry((*x1, y)).or_insert(0);
                *entry += 1;
            }
        } else if y1 == y2 {
            for x in if x1 < x2 { *x1..=*x2 } else { *x2..=*x1 } {
                let entry = all_coordinates.entry((x, *y1)).or_insert(0);
                *entry += 1;
            }
        } else {
            let xrange: Box<dyn Iterator<Item = _>> = if x1 < x2 {
                Box::new(*x1..=*x2)
            } else {
                Box::new((*x2..=*x1).rev())
            };
            let yrange: Box<dyn Iterator<Item = _>> = if y1 < y2 {
                Box::new(*y1..=*y2)
            } else {
                Box::new((*y2..=*y1).rev())
            };
            let range = xrange.zip(yrange);
            for (x, y) in range {
                let entry = all_coordinates.entry((x, y)).or_insert(0);
                *entry += 1;
            }
        }
    });

    all_coordinates.into_iter().filter(|e| e.1 > 1).count()
}

fn print_grid(coordinates: &HashMap<(u32, u32), i32>, width: usize, height: usize) {
    for row in 0..height {
        for column in 0..width {
            let c = match *coordinates.get(&(column as u32, row as u32)).unwrap_or(&0) {
                0 => '.',
                n => char::from_digit(n as u32, 10).unwrap(),
            };
            print!("{}", c);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn parsing() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(
            parse(&lines),
            vec![
                ((0, 9), (5, 9)),
                ((8, 0), (0, 8)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((6, 4), (2, 0)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2)),
            ]
        )
    }

    #[test]
    fn example1() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(h_v_overlaps(&lines), 5);
    }

    #[test]
    fn example2() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(h_v_d_overlaps(&lines), 12);
    }
}
