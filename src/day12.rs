use std::collections::HashMap;

use crate::input;

pub fn run() {
    let lines = input::get_lines("day12");

    let part1 = count_paths1(&lines);

    println!("part1: {:?}", part1);

    let part2 = count_paths2(&lines);

    println!("part2: {:?}", part2);
}

fn count_paths1(lines: &[String]) -> usize {
    count_paths(lines, true)
}

fn count_paths2(lines: &[String]) -> usize {
    count_paths(lines, false)
}

fn count_paths(lines: &[String], part1: bool) -> usize {
    let mut map = HashMap::<&str, Vec<&str>>::new();

    for line in lines {
        let mut split = line.split('-');
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        let entry = map.entry(first).or_insert_with(Vec::new);
        entry.push(second);
        let entry = map.entry(second).or_insert_with(Vec::new);
        entry.push(first);
    }

    let mut path = Vec::new();
    paths_from("start", &map, &mut path, part1)
}

fn paths_from<'a>(
    starting: &'a str,
    map: &'a HashMap<&'a str, Vec<&'a str>>,
    path: &mut Vec<&'a str>,
    mut seen_twice: bool,
) -> usize {
    if starting == "end" {
        return 1;
    }
    if starting.chars().all(|c| c.is_lowercase()) && path.contains(&starting) {
        if seen_twice || starting == "start" {
            return 0;
        }
        seen_twice = true;
    }
    path.push(starting);
    let result = map[&starting]
        .iter()
        .map(|&next| paths_from(next, map, path, seen_twice))
        .sum();
    path.pop();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example1() {
        let input =
            ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"].map(|s| s.to_string());

        assert_eq!(count_paths1(&input), 10);
    }

    #[test]
    fn larger_example1() {
        let input = [
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
        .map(|s| s.to_string());

        assert_eq!(count_paths1(&input), 226);
    }

    #[test]
    fn small_example2() {
        let input =
            ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"].map(|s| s.to_string());

        assert_eq!(count_paths2(&input), 36);
    }

    #[test]
    fn larger_example2() {
        let input = [
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
        .map(|s| s.to_string());

        assert_eq!(count_paths2(&input), 3509);
    }
}
