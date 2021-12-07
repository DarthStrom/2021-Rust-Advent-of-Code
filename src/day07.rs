use crate::input;

pub fn run() {
    let contents = input::get_contents("day07");

    let positions = parse(&contents);
    let (_destination, cost) = least_fuel(&positions);

    println!("part1: {:?}", cost);
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn cost(positions: &[i32], destination: i32) -> i32 {
    positions.iter().map(|x| (destination - x).abs()).sum()
}

fn least_fuel(positions: &[i32]) -> (i32, i32) {
    let mut result = (-1, i32::MAX);
    let max = positions.iter().max().unwrap();
    for x in 0..=*max {
        let cost = cost(positions, x);
        if cost < result.1 {
            result = (x, cost);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn example1() {
        let positions = parse(INPUT);

        assert_eq!(cost(&positions, 2), 37);
        assert_eq!(cost(&positions, 1), 41);
        assert_eq!(cost(&positions, 3), 39);
        assert_eq!(cost(&positions, 10), 71);

        assert_eq!(least_fuel(&positions), (2, 37));
    }
}
