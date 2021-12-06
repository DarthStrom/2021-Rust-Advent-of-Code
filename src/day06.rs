use std::collections::HashMap;

use crate::input;

pub fn run() {
    let contents = input::get_contents("day06");

    let day1 = advance_days(&parse(&contents), 80);

    println!("part1: {:?}", total(&day1));

    let day2 = advance_days(&parse(&contents), 256);

    println!("part2: {:?}", total(&day2));
}

fn parse(input: &str) -> HashMap<u32, usize> {
    input
        .trim()
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .fold(HashMap::new(), |acc, f| {
            let mut new = acc;
            let entry = new.entry(f).or_default();
            *entry += 1;
            new
        })
}

fn advance_day(current: &HashMap<u32, usize>) -> HashMap<u32, usize> {
    let mut ages: HashMap<u32, usize> = HashMap::new();

    for (&age, &count) in current {
        if age == 0 {
            let entry = ages.entry(6).or_default();
            *entry += count;
            ages.insert(8, count);
        } else {
            let entry = ages.entry(age - 1).or_default();
            *entry += count;
        }
    }

    ages
}

fn advance_days(current: &HashMap<u32, usize>, days: u32) -> HashMap<u32, usize> {
    let mut result: HashMap<u32, usize> = current.to_owned();
    let mut days_left = days;
    while days_left > 0 {
        result = advance_day(&result);
        days_left -= 1;
    }
    result
}

fn total(ages: &HashMap<u32, usize>) -> usize {
    ages.iter().map(|(_, count)| count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2\n";

    #[test]
    fn parsing() {
        let ages = parse(INPUT);

        assert_eq!(ages, HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]));
    }

    #[test]
    fn test_advance_day() {
        let initial = parse(INPUT);

        let day1 = advance_day(&initial);

        assert_eq!(day1, HashMap::from([(0, 1), (1, 1), (2, 2), (3, 1)]));

        let day2 = advance_day(&day1);

        assert_eq!(
            day2,
            HashMap::from([(0, 1), (1, 2), (2, 1), (6, 1), (8, 1)])
        );
    }

    #[test]
    fn test_multiple_days() {
        let initial = parse(INPUT);

        assert_eq!(
            advance_days(&initial, 2),
            HashMap::from([(0, 1), (1, 2), (2, 1), (6, 1), (8, 1)])
        );
    }

    #[test]
    fn example1() {
        let initial = parse(INPUT);

        let day18 = advance_days(&initial, 18);

        assert_eq!(
            day18,
            HashMap::from([
                (0, 3),
                (1, 5),
                (2, 3),
                (3, 2),
                (4, 2),
                (5, 1),
                (6, 5),
                (7, 1),
                (8, 4),
            ])
        );
        assert_eq!(total(&day18), 26);

        let day80 = advance_days(&initial, 80);

        assert_eq!(total(&day80), 5934);
    }

    #[test]
    fn example2() {
        assert_eq!(total(&advance_days(&parse(INPUT), 256)), 26984457539);
    }
}
