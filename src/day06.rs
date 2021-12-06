use crate::input;

pub fn run() {
    let contents = input::get_contents("day06");

    let day1 = advance_days(&parse(&contents), 80);

    println!("part1: {:?}", day1.len());
}

fn parse(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn advance_day(current: &[u32]) -> Vec<u32> {
    let mut spawn: Vec<u32> = vec![];
    let mut new = current
        .iter()
        .map(|l| {
            if l == &0 {
                spawn.push(8);
                6
            } else {
                l - 1
            }
        })
        .collect::<Vec<_>>();
    new.append(&mut spawn);

    new
}

fn advance_days(current: &[u32], days: u32) -> Vec<u32> {
    let mut result: Vec<u32> = current.to_vec();
    let mut days_left = days;
    while days_left > 0 {
        result = advance_day(&result);
        days_left -= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2\n";

    #[test]
    fn parsing() {
        let numbers = parse(INPUT);

        assert_eq!(numbers, vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn test_advance_day() {
        let initial = parse(INPUT);

        let day1 = advance_day(&initial);

        assert_eq!(day1, vec![2, 3, 2, 0, 1]);

        let day2 = advance_day(&day1);

        assert_eq!(day2, vec![1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn test_multiple_days() {
        let initial = parse(INPUT);

        assert_eq!(advance_days(&initial, 2), vec![1, 2, 1, 6, 0, 8]);
    }

    #[test]
    fn example1() {
        let initial = parse(INPUT);

        let day18 = advance_days(&initial, 18);

        assert_eq!(
            day18,
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );
        assert_eq!(day18.len(), 26);

        let day80 = advance_days(&initial, 80);

        assert_eq!(day80.len(), 5934);
    }
}
