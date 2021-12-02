use crate::input;

pub fn run() {
    let contents = input::get_lines("day01");

    let single_increases = increases(&contents, 1);
    println!("part1: {:?}", single_increases);

    let sliding_window_increases = increases(&contents, 3);
    println!("part2: {:?}", sliding_window_increases);
}

fn increases(report: &[String], window: usize) -> u32 {
    let depths = depths(report);
    let mut result = 0;
    depths.iter().enumerate().for_each(|(i, &depth)| {
        if i > window - 1 && depth > depths[i - window] {
            result += 1;
        }
    });
    result
}

fn depths(report: &[String]) -> Vec<u32> {
    report
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPORT: [&str; 10] = [
        "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
    ];

    #[test]
    fn example1() {
        assert_eq!(increases(&REPORT.map(|s| s.to_string()), 1), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(increases(&REPORT.map(|s| s.to_string()), 3), 5);
    }
}
