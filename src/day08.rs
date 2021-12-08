use crate::input;

pub fn run() {
    let notes = input::get_lines("day08");

    let part1 = count_unique_digits(&notes);

    println!("part1: {:?}", part1);
}

fn count_unique_digits(lines: &[String]) -> usize {
    lines.iter().map(|l| count_unique_in_line(l)).sum()
}

fn count_unique_in_line(line: &str) -> usize {
    let mut split = line.split(" | ");
    let _left = split.next().unwrap();
    let right = split.next().unwrap();
    right
        .split(' ')
        .filter(|&digit| [2, 3, 4, 7].contains(&digit.len()))
        .count()
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
}
