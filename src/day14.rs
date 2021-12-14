use std::collections::HashMap;

use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_lines("day14");

    let part1 = apply_pair_insertion(&input, 10);

    println!("part1: {:?}", part1);
}

fn apply_pair_insertion(instructions: &[String], steps: usize) -> usize {
    let mut polymer = get_template(instructions);
    let rules = get_rules(instructions);

    for _step in 1..=steps {
        let last = polymer.chars().last().unwrap();
        polymer = polymer
            .chars()
            .tuple_windows()
            .map(|(a, b)| {
                let new = rules.get(&format!("{}{}", a, b)).unwrap();
                format!("{}{}", a, new)
            })
            .join("");
        polymer.push(last);
        println!("{}", polymer);
    }

    let mut counts = HashMap::<char, usize>::new();

    polymer.chars().for_each(|c| {
        let mut entry = counts.entry(c).or_default();
        *entry += 1;
    });

    let max = counts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let min = counts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    max - min
}

fn get_template(instructions: &[String]) -> String {
    instructions[0].to_string()
}

fn get_rules(instructions: &[String]) -> HashMap<String, String> {
    let mut result = HashMap::new();
    instructions
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .for_each(|l| {
            let mut split = l.split(" -> ");
            result.insert(
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
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
    fn example() {
        let instructions = INPUT.map(|s| s.to_string());

        assert_eq!(apply_pair_insertion(&instructions, 10), 1588)
    }
}
