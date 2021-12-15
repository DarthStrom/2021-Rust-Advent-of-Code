use crate::input;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

pub fn run() {
    let input = input::get_lines("day15");

    let part1 = lowest_risk(&input);

    println!("part1: {:?}", part1);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    risk: u32,
}

fn lowest_risk(lines: &[String]) -> u32 {
    let all_nodes = get_nodes(lines);
    let start = &all_nodes[0][0];
    let goal = all_nodes.iter().last().unwrap().iter().last().unwrap();

    let result = dijkstra(
        &(start.x, start.y),
        |point| successors(point, &all_nodes),
        |point| point == &(goal.x, goal.y),
    )
    .expect("no path");

    result.1
}

fn successors(point: &(usize, usize), nodes: &[Vec<Node>]) -> Vec<((usize, usize), u32)> {
    let width = nodes[0].len();
    let height = nodes.len();
    let mut result = Vec::new();
    if point.0 > 0 {
        let potential = (point.0 - 1, point.1);
        result.push(potential);
    }
    if point.1 > 0 {
        let potential = (point.0, point.1 - 1);
        result.push(potential);
    }
    if point.0 < width - 1 {
        let potential = (point.0 + 1, point.1);
        result.push(potential);
    }
    if point.1 < height - 1 {
        let potential = (point.0, point.1 + 1);
        result.push(potential);
    }
    result
        .iter()
        .map(|&p| (p, nodes[p.1][p.0].risk))
        .collect_vec()
}

fn get_nodes(lines: &[String]) -> Vec<Vec<Node>> {
    let mut result = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let node = Node {
                x,
                y,
                risk: c.to_digit(10).unwrap(),
            };
            row.push(node);
        }
        result.push(row);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn example1() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(lowest_risk(&lines), 40);
    }
}
