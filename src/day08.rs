use itertools::Itertools;

use crate::input;

use std::collections::HashMap;

pub fn run() {
    let notes = input::get_lines("day08");

    let part1 = count_unique_digits(&notes);

    println!("part1: {:?}", part1);

    let part2 = get_sum_of_outputs(&notes);

    println!("part2: {:?}", part2);
}

fn count_unique_digits(lines: &[String]) -> usize {
    lines.iter().map(|l| count_unique_in_line(l)).sum()
}

fn count_unique_in_line(line: &str) -> usize {
    split_input(line)[1]
        .split(' ')
        .filter(|&digit| [2, 3, 4, 7].contains(&digit.len()))
        .count()
}

fn all_digits(line: &str) -> HashMap<String, u32> {
    let sides = split_input(line);
    let legend = sides[0].split(' ').collect::<Vec<_>>();

    let one = legend
        .iter()
        .find(|s| s.len() == 2)
        .map(|s| s.chars().sorted().collect::<String>())
        .unwrap();
    let four = legend
        .iter()
        .find(|s| s.len() == 4)
        .map(|s| s.chars().sorted().collect::<String>())
        .unwrap();
    let seven = legend
        .iter()
        .find(|s| s.len() == 3)
        .map(|s| s.chars().sorted().collect::<String>())
        .unwrap();
    let eight = legend
        .iter()
        .find(|s| s.len() == 7)
        .map(|s| s.chars().sorted().collect::<String>())
        .unwrap();

    let mut six_lengths = legend
        .iter()
        .filter(|s| s.len() == 6)
        .map(|s| s.chars().sorted().collect::<String>())
        .collect::<Vec<_>>();
    let nine_index = six_lengths
        .iter()
        .position(|s| four.chars().all(|c| s.contains(c)))
        .unwrap();
    let nine = six_lengths.swap_remove(nine_index);
    let zero_index = six_lengths
        .iter()
        .position(|s| one.chars().all(|c| s.contains(c)))
        .unwrap();
    let zero = six_lengths.swap_remove(zero_index);
    let six = six_lengths.remove(0);

    let mut five_lengths = legend
        .iter()
        .filter(|s| s.len() == 5)
        .map(|s| s.chars().sorted().collect::<String>())
        .collect::<Vec<_>>();
    let three_index = five_lengths
        .iter()
        .position(|s| one.chars().all(|c| s.contains(c)))
        .unwrap();
    let three = five_lengths.swap_remove(three_index);
    let five_index = five_lengths
        .iter()
        .position(|s| four.chars().filter(|&c| s.contains(c)).count() == 3)
        .unwrap();
    let five = five_lengths.swap_remove(five_index);
    let two = five_lengths.remove(0);

    HashMap::from([
        (zero, 0),
        (one, 1),
        (two, 2),
        (three, 3),
        (four, 4),
        (five, 5),
        (six, 6),
        (seven, 7),
        (eight, 8),
        (nine, 9),
    ])
}

fn split_input(line: &str) -> Vec<String> {
    line.split(" | ").map(|s| s.to_string()).collect::<Vec<_>>()
}

fn get_output(line: &str) -> u32 {
    let lookup = all_digits(line);

    split_input(line)[1]
        .split(' ')
        .map(|s| {
            lookup
                .get(&s.chars().sorted().collect::<String>())
                .unwrap()
                .to_string()
        })
        .join("")
        .parse::<u32>()
        .unwrap()
}

fn get_sum_of_outputs(lines: &[String]) -> u32 {
    lines.iter().map(|line| get_output(line)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: [&str; 1] =
        ["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"];

    const LARGE_EXAMPLE: [&str; 10] = [
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn example1() {
        assert_eq!(
            count_unique_digits(&LARGE_EXAMPLE.map(|s| s.to_string())),
            26
        );
    }

    #[test]
    fn short_example2() {
        assert_eq!(
            all_digits(SMALL_EXAMPLE[0]),
            HashMap::from([
                ("abcdeg".to_string(), 0),
                ("ab".to_string(), 1),
                ("acdfg".to_string(), 2),
                ("abcdf".to_string(), 3),
                ("abef".to_string(), 4),
                ("bcdef".to_string(), 5),
                ("bcdefg".to_string(), 6),
                ("abd".to_string(), 7),
                ("abcdefg".to_string(), 8),
                ("abcdef".to_string(), 9),
            ])
        );

        assert_eq!(get_output(SMALL_EXAMPLE[0]), 5353);
    }

    #[test]
    fn long_example2() {
        assert_eq!(
            get_sum_of_outputs(&LARGE_EXAMPLE.map(|l| l.to_string())),
            61229
        );
    }
}
