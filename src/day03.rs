use crate::input;

pub fn run() {
    let contents = input::get_lines("day03");
    let gamma = gamma(&contents);
    let epsilon = epsilon(&contents);
    let power_consumption = gamma * epsilon;

    println!("part1: {:?}", power_consumption);
}

fn gamma(strings: &[String]) -> u32 {
    let length = strings.len();
    let width = strings[0].len();
    let mut result_array = vec![];

    for i in 0..width {
        let ones: usize = strings
            .iter()
            .map(|s| {
                s.split("")
                    .filter(|s| !s.is_empty())
                    .nth(i)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .sum();
        if ones > length / 2 {
            result_array.push("1");
        } else {
            result_array.push("0");
        }
    }

    u32::from_str_radix(&result_array.join(""), 2).unwrap()
}

fn epsilon(strings: &[String]) -> u32 {
    let length = strings.len();
    let width = strings[0].len();
    let mut result_array = vec![];

    for i in 0..width {
        let ones: usize = strings
            .iter()
            .map(|s| {
                s.split("")
                    .filter(|s| !s.is_empty())
                    .nth(i)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .sum();
        if ones < length / 2 {
            result_array.push("1");
        } else {
            result_array.push("0");
        }
    }

    u32::from_str_radix(&result_array.join(""), 2).unwrap()
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
}
