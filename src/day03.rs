use crate::input;

pub fn run() {
    let contents = input::get_lines("day03");
    let gamma = gamma(&contents);
    let epsilon = epsilon(&contents);
    let power_consumption = gamma * epsilon;

    println!("part1: {:?}", power_consumption);

    let oxygen = oxygen(&contents);
    let co2_scrubber = co2_scrubber(&contents);
    let life_support = oxygen * co2_scrubber;

    println!("part2: {:?}", life_support);
}

fn gamma(strings: &[String]) -> u32 {
    calculate_rate(strings, |ones, zeroes| ones >= zeroes)
}

fn epsilon(strings: &[String]) -> u32 {
    calculate_rate(strings, |ones, zeroes| ones < zeroes)
}

fn calculate_rate<F>(strings: &[String], test: F) -> u32
where
    F: Fn(usize, usize) -> bool,
{
    let mut results = vec![];

    for i in 0..strings[0].len() {
        let ones: usize = count_ones(i, strings);
        let zeroes = strings.len() - ones;
        if test(ones, zeroes) {
            results.push("1");
        } else {
            results.push("0");
        }
    }

    u32::from_str_radix(&results.join(""), 2).unwrap()
}

fn oxygen(strings: &[String]) -> u32 {
    calculate_rating(strings, |ones, zeroes| ones >= zeroes)
}

fn co2_scrubber(strings: &[String]) -> u32 {
    calculate_rating(strings, |ones, zeroes| ones < zeroes)
}

fn calculate_rating<F>(strings: &[String], test: F) -> u32
where
    F: Fn(usize, usize) -> bool,
{
    let mut results = strings.to_vec();

    for i in 0..strings[0].len() {
        let ones: usize = count_ones(i, &results);
        let zeroes = results.len() - ones;

        let c = if test(ones, zeroes) { "1" } else { "0" };
        results = filter_by(results, c, i);

        if results.len() == 1 {
            return u32::from_str_radix(&results[0], 2).unwrap();
        }
    }

    panic!(
        "never narrowed to 1 result, ended up with {}",
        results.len()
    );
}

fn filter_by(strings: Vec<String>, c: &str, at: usize) -> Vec<String> {
    strings
        .into_iter()
        .filter(|s| s.split("").filter(|s| !s.is_empty()).nth(at).unwrap() == c)
        .collect::<Vec<String>>()
}

fn count_ones(at: usize, strings: &[String]) -> usize {
    strings
        .iter()
        .map(|s| {
            s.split("")
                .filter(|s| !s.is_empty())
                .nth(at)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn example1() {
        let strings = INPUT.map(|s| s.to_string());

        assert_eq!(gamma(&strings), 22);
        assert_eq!(epsilon(&strings), 9);
    }

    #[test]
    fn example2() {
        let strings = INPUT.map(|s| s.to_string());

        assert_eq!(oxygen(&strings), 23);
        assert_eq!(co2_scrubber(&strings), 10);
    }
}
