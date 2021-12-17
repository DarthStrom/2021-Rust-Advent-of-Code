use hex::decode;
use itertools::Itertools;

use crate::input;

pub fn run() {
    let input = input::get_contents("day16");

    let part1 = parse_input(&input).version_sum();

    println!("part1: {:?}", part1);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Literal {
    version: u32,
    value: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Operator {
    version: u32,
    packets: Vec<Packet>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal(l) => l.version,
            Packet::Operator(o) => {
                o.version + o.packets.iter().map(|p| p.version_sum()).sum::<u32>()
            }
        }
    }
}

fn parse_input(input: &str) -> Packet {
    let decoded = decode(input)
        .unwrap()
        .iter()
        .map(|b| format!("{:08b}", b))
        .join("");

    let (packet, _length) = parse_packet(&decoded.chars().collect_vec());
    packet
}

fn parse_packet(chars: &[char]) -> (Packet, usize) {
    let packet_type = chars.iter().skip(3).take(3).join("");
    match packet_type.as_str() {
        "100" => parse_literal(chars),
        _ => parse_operator(chars),
    }
}

fn parse_literal(chars: &[char]) -> (Packet, usize) {
    let version = u32::from_str_radix(&chars[..3].iter().join(""), 2).unwrap();
    let bits = chars.iter().skip(6).collect_vec();
    let mut bit_string = String::new();
    let mut done = false;
    let mut nibble = 0;

    while !done {
        let start = nibble * 5;
        let indicator = bits[nibble * 5];
        if indicator == &'0' {
            done = true
        }
        let nibble_string = bits[start + 1..=start + 4].iter().join("");
        bit_string.push_str(&nibble_string);
        nibble += 1;
    }
    let value = usize::from_str_radix(&bit_string, 2).unwrap();

    let length = 6 + bit_string.len() + nibble;
    (Packet::Literal(Literal { version, value }), length)
}

fn parse_operator(chars: &[char]) -> (Packet, usize) {
    let version = u32::from_str_radix(&chars[..3].iter().join(""), 2).unwrap();
    let bits = chars.iter().skip(6).collect_vec();
    let mut total_length = 7;

    let mut packets = vec![];

    let mut parsed_length = 0;
    let length_type = bits[0];
    if length_type == &'0' {
        let length = usize::from_str_radix(&bits[1..=15].iter().join(""), 2).unwrap();
        total_length += length + 15;

        while parsed_length < length {
            let (packet, length) = parse_packet(&chars[22 + parsed_length..]);
            parsed_length += length;
            packets.push(packet);
        }
    } else {
        let sub_packet_count = usize::from_str_radix(&bits[1..=11].iter().join(""), 2).unwrap();

        for _ in 0..sub_packet_count {
            let (packet, length) = parse_packet(&chars[18 + parsed_length..]);
            parsed_length += length;
            packets.push(packet);
        }
        total_length += parsed_length + 11;
    }

    (
        Packet::Operator(Operator { version, packets }),
        total_length,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parsing() {
        let input = "D2FE28";

        assert_eq!(
            parse_input(input),
            Packet::Literal(Literal {
                version: 6,
                value: 2021
            })
        );
    }

    #[test]
    fn operator_parsing1() {
        let input = "38006F45291200";

        assert_eq!(
            parse_input(input),
            Packet::Operator(Operator {
                version: 1,
                packets: vec![
                    Packet::Literal(Literal {
                        version: 6,
                        value: 10
                    }),
                    Packet::Literal(Literal {
                        version: 2,
                        value: 20
                    })
                ]
            })
        )
    }

    #[test]
    fn operator_parsing2() {
        let input = "EE00D40C823060";

        assert_eq!(
            parse_input(input),
            Packet::Operator(Operator {
                version: 7,
                packets: vec![
                    Packet::Literal(Literal {
                        version: 2,
                        value: 1
                    }),
                    Packet::Literal(Literal {
                        version: 4,
                        value: 2
                    }),
                    Packet::Literal(Literal {
                        version: 1,
                        value: 3
                    })
                ]
            })
        )
    }

    #[test]
    fn nested_1() {
        let input = "8A004A801A8002F478";

        let expected = Packet::Operator(Operator {
            version: 4,
            packets: vec![Packet::Operator(Operator {
                version: 1,
                packets: vec![Packet::Operator(Operator {
                    version: 5,
                    packets: vec![Packet::Literal(Literal {
                        version: 6,
                        value: 15,
                    })],
                })],
            })],
        });

        assert_eq!(parse_input(input), expected);

        assert_eq!(expected.version_sum(), 16);
    }

    #[test]
    fn nested_2() {
        let input = "620080001611562C8802118E34";

        assert_eq!(parse_input(input).version_sum(), 12);
    }

    #[test]
    fn nested_3() {
        let input = "C0015000016115A2E0802F182340";

        assert_eq!(parse_input(input).version_sum(), 23);
    }

    #[test]
    fn nested_4() {
        let input = "A0016C880162017C3686B18A3D4780";

        assert_eq!(parse_input(input).version_sum(), 31)
    }
}
