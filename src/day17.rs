pub fn run() {
    let target = (Point { x: 60, y: -171 }, Point { x: 94, y: -136 });

    let (_, part1) = high_shot(target);

    println!("part1: {:?}", part1);

    let part2 = all_valid(target).len();

    println!("part2: {:?}", part2);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn high_shot(target: (Point, Point)) -> (Point, i32) {
    let mut highest = 0;
    let mut result = Point { x: 0, y: 0 };
    for x in 0..=200 {
        for y in 0..=200 {
            if let Some((_, high)) = shoot(Point { x, y }, target) {
                if high > highest {
                    highest = high;
                    result = Point { x, y }
                }
            }
        }
    }
    (result, highest)
}

fn all_valid(target: (Point, Point)) -> Vec<Point> {
    let mut result = vec![];
    for x in 0..=200 {
        for y in -200..200 {
            if shoot(Point { x, y }, target).is_some() {
                result.push(Point { x, y })
            }
        }
    }
    result
}

fn shoot(initial_velocity: Point, target: (Point, Point)) -> Option<(Point, i32)> {
    let mut x = initial_velocity.x;
    let mut x_velocity = initial_velocity.x;
    let mut y = initial_velocity.y;
    let mut y_velocity = initial_velocity.y;
    let mut high_y = y;
    while x <= (target.1).x && y >= (target.0).y {
        if y > high_y {
            high_y = y;
        }
        if x >= (target.0).x && y <= (target.1).y {
            return Some((Point { x, y }, high_y));
        }
        match x_velocity {
            1.. => x_velocity -= 1,
            0 => (),
            _ => x_velocity += 1,
        }
        y_velocity -= 1;
        x += x_velocity;
        y += y_velocity;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET: (Point, Point) = (Point { x: 20, y: -10 }, Point { x: 30, y: -5 });

    #[test]
    fn example1() {
        assert_eq!(
            shoot(Point { x: 7, y: 2 }, TARGET),
            Some((Point { x: 28, y: -7 }, 3))
        );

        assert_eq!(
            shoot(Point { x: 6, y: 3 }, TARGET),
            Some((Point { x: 21, y: -9 }, 6))
        );

        assert_eq!(
            shoot(Point { x: 9, y: 0 }, TARGET),
            Some((Point { x: 30, y: -6 }, 0))
        );

        assert_eq!(shoot(Point { x: 17, y: 0 - 4 }, TARGET), None);
    }

    #[test]
    fn getting_the_high_shot() {
        assert_eq!(high_shot(TARGET), (Point { x: 6, y: 9 }, 45));
    }

    #[test]
    fn example2() {
        assert_eq!(all_valid(TARGET).len(), 112);
    }
}
