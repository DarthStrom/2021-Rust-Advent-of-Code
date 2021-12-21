use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_contents("day19");

    println!("part1: {:?}", input);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coords(i32, i32, i32);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scanner {
    id: u32,
    beacons: Vec<Coords>,
}

fn get_scanners(input: &[String]) -> Vec<Scanner> {
    let mut result = vec![];
    let mut id = 0;
    let mut beacons = vec![];
    println!("{:?}", input);
    for line in input {
        if line.starts_with("---") {
            id = parse_id(line)
        } else if line.is_empty() {
            let scanner = Scanner {
                id,
                beacons: beacons.clone(),
            };
            result.push(scanner);
            beacons.clear();
        } else {
            beacons.push(parse_beacon(line));
        }
    }
    result.push(Scanner { id, beacons });
    result
}

fn parse_id(line: &str) -> u32 {
    let split = line.split(" ");
    let words = split.collect_vec();
    words[2].parse::<u32>().unwrap()
}

fn parse_beacon(line: &str) -> Coords {
    let mut split = line.split(",");
    Coords(
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
        split.next().unwrap().parse::<i32>().unwrap(),
    )
}

fn find_overlapping(scanner_a: &Scanner, scanner_b: &Scanner) -> Vec<Coords> {
    let mut overlapping = vec![];
    for x in -1000..=1000 {
        for y in -1000..=1000 {
            for z in -1000..=1000 {
                let translated = scanner_b
                    .beacons
                    .iter()
                    .map(|c| Coords(c.0 + x, c.1 + y, c.2 + z))
                    .collect_vec();
                let this_overlapping = scanner_a
                    .beacons
                    .iter()
                    .cloned()
                    .filter(|c| translated.contains(c))
                    .collect_vec();
                if this_overlapping.len() > overlapping.len() {
                    overlapping = this_overlapping;
                }
            }
        }
    }
    overlapping
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = input::get_lines("day19example");

        assert_eq!(
            get_scanners(&input),
            vec![
                Scanner {
                    id: 0,
                    beacons: vec![
                        Coords(404, -588, -901),
                        Coords(528, -643, 409),
                        Coords(-838, 591, 734),
                        Coords(390, -675, -793),
                        Coords(-537, -823, -458),
                        Coords(-485, -357, 347),
                        Coords(-345, -311, 381),
                        Coords(-661, -816, -575),
                        Coords(-876, 649, 763),
                        Coords(-618, -824, -621),
                        Coords(553, 345, -567),
                        Coords(474, 580, 667),
                        Coords(-447, -329, 318),
                        Coords(-584, 868, -557),
                        Coords(544, -627, -890),
                        Coords(564, 392, -477),
                        Coords(455, 729, 728),
                        Coords(-892, 524, 684),
                        Coords(-689, 845, -530),
                        Coords(423, -701, 434),
                        Coords(7, -33, -71),
                        Coords(630, 319, -379),
                        Coords(443, 580, 662),
                        Coords(-789, 900, -551),
                        Coords(459, -707, 401),
                    ]
                },
                Scanner {
                    id: 1,
                    beacons: vec![
                        Coords(686, 422, 578),
                        Coords(605, 423, 415),
                        Coords(515, 917, -361),
                        Coords(-336, 658, 858),
                        Coords(95, 138, 22),
                        Coords(-476, 619, 847),
                        Coords(-340, -569, -846),
                        Coords(567, -361, 727),
                        Coords(-460, 603, -452),
                        Coords(669, -402, 600),
                        Coords(729, 430, 532),
                        Coords(-500, -761, 534),
                        Coords(-322, 571, 750),
                        Coords(-466, -666, -811),
                        Coords(-429, -592, 574),
                        Coords(-355, 545, -477),
                        Coords(703, -491, -529),
                        Coords(-328, -685, 520),
                        Coords(413, 935, -424),
                        Coords(-391, 539, -444),
                        Coords(586, -435, 557),
                        Coords(-364, -763, -893),
                        Coords(807, -499, -711),
                        Coords(755, -354, -619),
                        Coords(553, 889, -390),
                    ]
                },
                Scanner {
                    id: 2,
                    beacons: vec![
                        Coords(649, 640, 665),
                        Coords(682, -795, 504),
                        Coords(-784, 533, -524),
                        Coords(-644, 584, -595),
                        Coords(-588, -843, 648),
                        Coords(-30, 6, 44),
                        Coords(-674, 560, 763),
                        Coords(500, 723, -460),
                        Coords(609, 671, -379),
                        Coords(-555, -800, 653),
                        Coords(-675, -892, -343),
                        Coords(697, -426, -610),
                        Coords(578, 704, 681),
                        Coords(493, 664, -388),
                        Coords(-671, -858, 530),
                        Coords(-667, 343, 800),
                        Coords(571, -461, -707),
                        Coords(-138, -166, 112),
                        Coords(-889, 563, -600),
                        Coords(646, -828, 498),
                        Coords(640, 759, 510),
                        Coords(-630, 509, 768),
                        Coords(-681, -892, -333),
                        Coords(673, -379, -804),
                        Coords(-742, -814, -386),
                        Coords(577, -820, 562),
                    ]
                },
                Scanner {
                    id: 3,
                    beacons: vec![
                        Coords(-589, 542, 597),
                        Coords(605, -692, 669),
                        Coords(-500, 565, -823),
                        Coords(-660, 373, 557),
                        Coords(-458, -679, -417),
                        Coords(-488, 449, 543),
                        Coords(-626, 468, -788),
                        Coords(338, -750, -386),
                        Coords(528, -832, -391),
                        Coords(562, -778, 733),
                        Coords(-938, -730, 414),
                        Coords(543, 643, -506),
                        Coords(-524, 371, -870),
                        Coords(407, 773, 750),
                        Coords(-104, 29, 83),
                        Coords(378, -903, -323),
                        Coords(-778, -728, 485),
                        Coords(426, 699, 580),
                        Coords(-438, -605, -362),
                        Coords(-469, -447, -387),
                        Coords(509, 732, 623),
                        Coords(647, 635, -688),
                        Coords(-868, -804, 481),
                        Coords(614, -800, 639),
                        Coords(595, 780, -596),
                    ]
                },
                Scanner {
                    id: 4,
                    beacons: vec![
                        Coords(727, 592, 562),
                        Coords(-293, -554, 779),
                        Coords(441, 611, -461),
                        Coords(-714, 465, -776),
                        Coords(-743, 427, -804),
                        Coords(-660, -479, -426),
                        Coords(832, -632, 460),
                        Coords(927, -485, -438),
                        Coords(408, 393, -506),
                        Coords(466, 436, -512),
                        Coords(110, 16, 151),
                        Coords(-258, -428, 682),
                        Coords(-393, 719, 612),
                        Coords(-211, -452, 876),
                        Coords(808, -476, -593),
                        Coords(-575, 615, 604),
                        Coords(-485, 667, 467),
                        Coords(-680, 325, -822),
                        Coords(-627, -443, -432),
                        Coords(872, -547, -609),
                        Coords(833, 512, 582),
                        Coords(807, 604, 487),
                        Coords(839, -516, 451),
                        Coords(891, -625, 532),
                        Coords(-652, -548, -490),
                        Coords(30, -46, -14),
                    ]
                },
            ]
        );
    }

    // #[test]
    // fn overlapping_cubes() {
    //     let input = input::get_lines("day19example");

    //     let scanners = get_scanners(&input);

    //     assert_eq!(
    //         find_overlapping(&scanners[0], &scanners[1]),
    //         vec![
    //             Coords(-618, -824, -621),
    //             Coords(-537, -823, -458),
    //             Coords(-447, -329, 318),
    //             Coords(404, -588, -901),
    //             Coords(544, -627, -890),
    //             Coords(528, -643, 409),
    //             Coords(-661, -816, -575),
    //             Coords(390, -675, -793),
    //             Coords(423, -701, 434),
    //             Coords(-345, -311, 381),
    //             Coords(459, -707, 401),
    //             Coords(-485, -357, 347),
    //         ]
    //     )
    // }
}
