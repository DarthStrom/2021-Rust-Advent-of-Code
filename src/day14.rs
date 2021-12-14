use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

use crate::input;

pub fn run() {
    let input = input::get_lines("day14");

    let part1 = apply_pair_insertion(&input, 10);

    println!("part1: {:?}", part1);

    let part2 = apply_pair_insertion(&input, 40);

    println!("part2: {:?}", part2);
}

fn apply_pair_insertion(instructions: &[String], steps: usize) -> usize {
    let mut counts = HashMap::<char, usize>::new();
    let mut polymer = get_template(instructions);
    let rules = get_rules(instructions);

    for _step in 1..=steps {
        let mut new_polymer = HashMap::new();
        for ((c1, c2), count) in polymer {
            if let Some(&mid) = rules.get(&(c1, c2)) {
                *new_polymer.entry((c1, mid)).or_default() += count;
                *new_polymer.entry((mid, c2)).or_default() += count;
            } else {
                *new_polymer.entry((c1, c2)).or_default() += count;
            }
        }
        polymer = new_polymer;
    }

    for ((c1, c2), count) in polymer {
        *counts.entry(c1).or_default() += count;
        *counts.entry(c2).or_default() += count;
    }

    counts.remove(&'x');

    if let MinMaxResult::MinMax(min, max) = counts.values().minmax() {
        // divide by 2 since each char shows up twice per appearance
        // e.g. ABC will result in [xA, AB, BC, Cx]
        (*max - *min) / 2
    } else {
        0
    }
}

fn get_template(instructions: &[String]) -> HashMap<(char, char), usize> {
    // add buffer so that end chars show up twice like the middle ones
    format!("x{}x", instructions[0])
        .chars()
        .tuple_windows()
        .counts()
}

fn get_rules(instructions: &[String]) -> HashMap<(char, char), char> {
    let mut result = HashMap::new();
    instructions
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .for_each(|l| {
            let mut split = l.split(" -> ");
            let mut chars = split.next().unwrap().chars();
            result.insert(
                (chars.next().unwrap(), chars.next().unwrap()),
                split.next().unwrap().chars().next().unwrap(),
            );
        });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 18] = [
        "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C",
        "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N",
        "CN -> C",
    ];

    #[test]
    fn example1() {
        let instructions = INPUT.map(|s| s.to_string());

        assert_eq!(apply_pair_insertion(&instructions, 10), 1588);
    }

    #[test]
    fn example2() {
        let instructions = INPUT.map(|s| s.to_string());

        assert_eq!(apply_pair_insertion(&instructions, 40), 2188189693529);
    }
}
