use crate::input;

pub fn run() {
    let contents = input::get_lines("day01");

    let increases = count_increases(&contents);
    println!("part1: {:?}", increases);

    let sliding_window_increases = get_sliding_window_increases(&contents);
    println!("part2: {:?}", sliding_window_increases);
}

fn count_increases(report: &[String]) -> u32 {
    let depths = depths(report);
    let mut result = 0;
    depths.iter().enumerate().for_each(|(i, &depth)| {
        if i > 0 && depth > depths[i - 1] {
            result += 1;
        }
    });
    result
}

fn get_sliding_window_increases(report: &[String]) -> u32 {
    let depths = depths(report);
    let mut result = 0;
    depths.iter().enumerate().for_each(|(i, &depth)| {
        if i > 2 && depth > depths[i - 3] {
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
        assert_eq!(count_increases(&REPORT.map(|s| s.to_string())), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(
            get_sliding_window_increases(&REPORT.map(|s| s.to_string())),
            5
        );
    }
}
