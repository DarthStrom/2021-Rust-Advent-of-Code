use crate::input;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

pub fn run() {
    let input = input::get_lines("day15");

    let part1 = lowest_risk(&input);

    println!("part1: {:?}", part1);

    let part2 = lowest_risk_full(&input);

    println!("part2: {:?}", part2);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    risk: u32,
}

fn lowest_risk(lines: &[String]) -> u32 {
    let all_nodes = get_nodes(lines);

    solve(&all_nodes)
}

fn lowest_risk_full(lines: &[String]) -> u32 {
    let first_nodes = get_nodes(lines);
    let all_nodes = expound(&first_nodes);

    solve(&all_nodes)
}

fn solve(nodes: &[Vec<Node>]) -> u32 {
    let start = &nodes[0][0];
    let goal = nodes.iter().last().unwrap().iter().last().unwrap();

    let result = dijkstra(
        &(start.x, start.y),
        |point| successors(point, nodes),
        |point| point == &(goal.x, goal.y),
    )
    .expect("no path");

    result.1
}

fn expound(original: &[Vec<Node>]) -> Vec<Vec<Node>> {
    let original_width = original[0].len();
    let original_height = original.len();
    let mut result = vec![
        vec![
            Node {
                x: 0,
                y: 0,
                risk: 0
            };
            original_width * 5
        ];
        original_height * 5
    ];
    for (original_y, row) in original.iter().enumerate() {
        for (original_x, node) in row.iter().enumerate() {
            for tile_y in 0..5 {
                for tile_x in 0..5 {
                    let x = original_x + tile_x * original_width;
                    let y = original_y + tile_y * original_height;
                    let mut risk = (node.risk + tile_x as u32 + tile_y as u32) % 9 + 9;
                    if risk > 9 {
                        risk -= 9
                    }
                    result[y][x] = Node { x, y, risk }
                }
            }
        }
    }
    result
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

    #[test]
    fn example2() {
        let lines = INPUT.map(|s| s.to_string());

        assert_eq!(lowest_risk_full(&lines), 315);
    }
}
