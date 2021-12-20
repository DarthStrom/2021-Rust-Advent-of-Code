use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_lines("day18");

    println!("part1: {:?}", input);
}

fn final_sum(input: &[String]) -> Vec<(u32, u32)> {
    println!("input: {:?}", input);
    parse(input)
        .into_iter()
        .reduce(|a, b| add(&a, &b))
        .unwrap()
        .to_vec()
}

fn parse(input: &[String]) -> Vec<Vec<(u32, u32)>> {
    input.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<(u32, u32)> {
    let mut result = vec![];
    let mut depth = 0;
    for char in line.chars() {
        match char {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            n => result.push((n.to_digit(10).unwrap(), depth)),
        }
    }
    result
}

fn add(first: &[(u32, u32)], second: &[(u32, u32)]) -> Vec<(u32, u32)> {
    println!("  {:?}", first);
    println!("+ {:?}", second);
    let mut result = first.iter().map(|n| (n.0, n.1 + 1)).collect_vec();
    let mut other = second.iter().map(|n| (n.0, n.1 + 1)).collect_vec();
    result.append(&mut other);

    let mut needs_split = true;
    let mut needs_explode = true;
    while needs_split || needs_explode {
        needs_explode = result.iter().any(|n| n.1 > 4);
        while needs_explode {
            result = explode(&result);
            println!("exploded");
            needs_explode = result.iter().any(|n| n.1 > 4);
        }
        needs_split = result.iter().any(|n| n.0 > 9);
        while needs_split {
            let i = result.iter().position(|n| n.0 > 9).unwrap();
            let existing = result.remove(i);
            let new = split(existing.0);
            result.insert(i, (new.1, existing.1 + 1));
            result.insert(i, (new.0, existing.1 + 1));
            println!("split");
            needs_split = result.iter().any(|n| n.0 > 9);
        }
        needs_explode = result.iter().any(|n| n.1 > 4);
        needs_split = result.iter().any(|n| n.0 > 9);
        println!("{:?}", result);
    }

    println!();

    result
}

fn explode(number: &[(u32, u32)]) -> Vec<(u32, u32)> {
    let mut result = number.to_vec();
    if let Some(i) = number.iter().position(|n| n.1 > 4) {
        if i == 0 {
            result.remove(0);
        } else {
            let (old, _) = result[i];
            let (left, depth) = result[i - 1];
            result[i - 1] = (left + old, depth);
            result.remove(i);
        }

        if i == number.len() - 2 {
            let (_, depth) = result[i];
            result[i] = (0, depth - 1);
        } else {
            let (old, depth) = result[i];
            result[i] = (0, depth - 1);
            let right = result[i + 1];
            result[i + 1] = (old + right.0, right.1);
        }
    }
    result
}

fn split(n: u32) -> (u32, u32) {
    (n / 2, if n % 2 == 0 { n / 2 } else { n / 2 + 1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn parsing() {
    //     let input = ["[1,2]", "[[1,2],3]", "[9,[8,7]]"].map(|s| s.to_string());

    //     assert_eq!(
    //         parse(&input),
    //         vec![
    //             vec![(1, 1), (2, 1)],
    //             vec![(1, 2), (2, 2), (3, 1)],
    //             vec![(9, 1), (8, 2), (7, 2)],
    //         ]
    //     );
    // }

    // #[test]
    // fn exploding() {
    //     assert_eq!(
    //         explode(&[(9, 5), (8, 5), (1, 4), (2, 3), (3, 2), (4, 1)]),
    //         vec![(0, 4), (9, 4), (2, 3), (3, 2), (4, 1)]
    //     );
    //     assert_eq!(
    //         explode(&[(7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 5)]),
    //         vec![(7, 1), (6, 2), (5, 3), (7, 4), (0, 4)]
    //     );
    //     assert_eq!(
    //         explode(&[(6, 2), (5, 3), (4, 4), (3, 5), (2, 5), (1, 1)]),
    //         vec![(6, 2), (5, 3), (7, 4), (0, 4), (3, 1)]
    //     );

    //     let mut result = explode(&[
    //         (3, 2),
    //         (2, 3),
    //         (1, 4),
    //         (7, 5),
    //         (3, 5),
    //         (6, 2),
    //         (5, 3),
    //         (4, 4),
    //         (3, 5),
    //         (2, 5),
    //     ]);
    //     assert_eq!(
    //         result,
    //         vec![
    //             (3, 2),
    //             (2, 3),
    //             (8, 4),
    //             (0, 4),
    //             (9, 2),
    //             (5, 3),
    //             (4, 4),
    //             (3, 5),
    //             (2, 5)
    //         ]
    //     );

    //     result = explode(&result);
    //     assert_eq!(
    //         result,
    //         vec![
    //             (3, 2),
    //             (2, 3),
    //             (8, 4),
    //             (0, 4),
    //             (9, 2),
    //             (5, 3),
    //             (7, 4),
    //             (0, 4)
    //         ]
    //     )
    // }

    // #[test]
    // fn splitting() {
    //     assert_eq!(split(10), (5, 5));
    //     assert_eq!(split(11), (5, 6));
    //     assert_eq!(split(12), (6, 6));
    // }

    // #[test]
    // fn addition() {
    //     assert_eq!(
    //         add(
    //             &[
    //                 (4, 4),
    //                 (3, 4),
    //                 (4, 3),
    //                 (4, 2),
    //                 (7, 2),
    //                 (8, 4),
    //                 (4, 4),
    //                 (9, 3)
    //             ],
    //             &[(1, 1), (1, 1)]
    //         ),
    //         vec![
    //             (0, 4),
    //             (7, 4),
    //             (4, 3),
    //             (7, 4),
    //             (8, 4),
    //             (6, 4),
    //             (0, 4),
    //             (8, 2),
    //             (1, 2)
    //         ]
    //     );
    // }

    // #[test]
    // fn final_summing1() {
    //     let input = ["[1,1]", "[2,2]", "[3,3]", "[4,4]"].map(|s| s.to_string());

    //     assert_eq!(
    //         final_sum(&input),
    //         vec![
    //             (1, 4),
    //             (1, 4),
    //             (2, 4),
    //             (2, 4),
    //             (3, 3),
    //             (3, 3),
    //             (4, 2),
    //             (4, 2)
    //         ]
    //     );
    // }

    // #[test]
    // fn final_summing2() {
    //     let input = [
    //         "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
    //         "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    //         "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
    //         "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
    //         "[7,[5,[[3,8],[1,4]]]]",
    //         "[[2,[2,2]],[8,[8,1]]]",
    //         "[2,9]",
    //         "[1,[[[9,3],9],[[9,0],[0,7]]]]",
    //         "[[[5,[7,4]],7],1]",
    //         "[[[[4,2],2],6],[8,7]]",
    //     ]
    //     .map(|s| s.to_string());

    //     assert_eq!(final_sum(&input[..2]), vec![]);

    //     assert_eq!(
    //         final_sum(&input),
    //         vec![
    //             (8, 4),
    //             (7, 4),
    //             (7, 4),
    //             (7, 4),
    //             (8, 4),
    //             (6, 4),
    //             (7, 4),
    //             (7, 4),
    //             (0, 4),
    //             (7, 4),
    //             (6, 4),
    //             (6, 4),
    //             (8, 3),
    //             (7, 3)
    //         ]
    //     );
    // }
}
