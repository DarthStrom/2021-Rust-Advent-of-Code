use crate::input;

pub fn run() {
    let contents = input::get_contents("day01");
    println!("part1: {}", contents);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(1, 2)
    }
}
