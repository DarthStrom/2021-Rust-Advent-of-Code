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

    #[test]
    fn example1() {
        let report = vec![
            "199".to_string(),
            "200".to_string(),
            "208".to_string(),
            "210".to_string(),
            "200".to_string(),
            "207".to_string(),
            "240".to_string(),
            "269".to_string(),
            "260".to_string(),
            "263".to_string(),
        ];
        assert_eq!(count_increases(&report), 7);
    }

    #[test]
    fn example2() {
        let report = vec![
            "199".to_string(),
            "200".to_string(),
            "208".to_string(),
            "210".to_string(),
            "200".to_string(),
            "207".to_string(),
            "240".to_string(),
            "269".to_string(),
            "260".to_string(),
            "263".to_string(),
        ];
        assert_eq!(get_sliding_window_increases(&report), 5);
    }
}
