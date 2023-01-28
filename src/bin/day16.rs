#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::vec::Vec;
use advent_lib::read::read_input;

lazy_static! {
    static ref HEX2BIN: HashMap<char, &'static str> = [
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ].iter().cloned().collect();
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Literal(u64),
    Operator(Vec<Box<Packet>>),
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: Payload,
}

fn bitstream<'a>(hex: &'a String) -> impl Iterator<Item=char> + 'a {
    hex
        .chars()
        .flat_map(|c| HEX2BIN[&c].chars())
}

fn get_bits_to_int(n: usize, bitstream: &mut dyn Iterator<Item=char>) -> u64 {
    assert!(n <= 64);
    let bitstr: String = bitstream.take(n).collect();
    assert!(bitstr.len() == n);
    u64::from_str_radix(&bitstr, 2).unwrap()
}

fn parse_packet(bitstream: &mut dyn Iterator<Item=char>) -> Box<Packet> {
    let version = get_bits_to_int(3, bitstream) as u8;
    let type_id = get_bits_to_int(3, bitstream) as u8;
    match type_id {
        4 => {
            let mut val: u64 = 0;
            while bitstream.next().unwrap() == '1' {
                val = (val << 4) | get_bits_to_int(4, bitstream);
            }
            val = (val << 4) | get_bits_to_int(4, bitstream);
            Box::new(Packet {
                version,
                type_id,
                payload: Payload::Literal(val),
            })
        },
        _ => {
            let mut subpackets: Vec<Box<Packet>> = Vec::new();
            if bitstream.next().unwrap() == '0' {
                let nbits = get_bits_to_int(15, bitstream);
                let mut substring = bitstream.take(nbits as usize).peekable();
                while let Some(_) = substring.peek() {
                    subpackets.push(parse_packet(&mut substring));
                }
            } else {
                let npackets = get_bits_to_int(11, bitstream);
                for _ in 0..npackets {
                    subpackets.push(parse_packet(bitstream));
                }
            }
            Box::new(Packet {
                version,
                type_id,
                payload: Payload::Operator(subpackets),
            })
        },
    }
}

fn sum_versions(pkt: Box<Packet>) -> u64 {
    let mut sum = (*pkt).version as u64;
    match (*pkt).payload {
        Payload::Operator(subpackets) => {
            for p in subpackets {
                sum += sum_versions(p);
            }
        },
        _ => {},
    }
    sum
}

fn value_of(pkt: &Box<Packet>) -> u64 {
    match &(*pkt).payload {
        Payload::Literal(val) => *val,
        Payload::Operator(subpackets) => {
            match (*pkt).type_id {
                0 => subpackets.iter().map(|p| value_of(p)).sum(),
                1 => subpackets.iter().map(|p| value_of(p)).product(),
                2 => subpackets.iter().map(|p| value_of(p)).min().unwrap(),
                3 => subpackets.iter().map(|p| value_of(p)).max().unwrap(),
                5 => if value_of(&subpackets[0]) > value_of(&subpackets[1]) { 1 } else { 0 },
                6 => if value_of(&subpackets[0]) < value_of(&subpackets[1]) { 1 } else { 0 },
                7 => if value_of(&subpackets[0]) == value_of(&subpackets[1]) { 1 } else { 0 },
                _ => panic!(),
            }
        }
    }
}

fn part1(input: &Vec<String>) -> u64 {
    let mut bitstream = bitstream(&input[0]);
    let pkt = parse_packet(&mut bitstream);
    sum_versions(pkt)
}

fn part2(input: &Vec<String>) -> u64 {
    let mut bitstream = bitstream(&input[0]);
    let pkt = parse_packet(&mut bitstream);
    value_of(&pkt)
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test() {
        let bstrm:String = bitstream(&String::from("ABC0123")).collect();
        assert_eq!(bstrm, "1010101111000000000100100011");

        let hex = &String::from("D2FE28");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(*pkt, Packet{version: 6, type_id: 4, payload: Payload::Literal(2021)});

        let hex = &String::from("38006F45291200");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(*pkt, Packet {
            version: 1,
            type_id: 6,
            payload: Payload::Operator(
                vec![
                    Box::new(
                        Packet { version: 6, type_id: 4, payload: Payload::Literal(10) }
                    ),
                    Box::new(
                        Packet { version: 2, type_id: 4, payload: Payload::Literal(20) }
                    ),
                ]
            ),
        });

        let hex = &String::from("EE00D40C823060");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(*pkt, Packet {
            version: 7,
            type_id: 3,
            payload: Payload::Operator(
                vec![
                    Box::new(
                        Packet { version: 2, type_id: 4, payload: Payload::Literal(1) }
                    ),
                    Box::new(
                        Packet { version: 4, type_id: 4, payload: Payload::Literal(2) }
                    ),
                    Box::new(
                        Packet { version: 1, type_id: 4, payload: Payload::Literal(3) }
                    ),
                ]
            ),
        });

        let hex = &String::from("8A004A801A8002F478");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(sum_versions(pkt), 16);

        let hex = &String::from("620080001611562C8802118E34");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(sum_versions(pkt), 12);

        let hex = &String::from("C0015000016115A2E0802F182340");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(sum_versions(pkt), 23);

        let hex = &String::from("A0016C880162017C3686B18A3D4780");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(sum_versions(pkt), 31);

        let hex = &String::from("C200B40A82");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 3);

        let hex = &String::from("04005AC33890");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 54);

        let hex = &String::from("880086C3E88112");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 7);

        let hex = &String::from("CE00C43D881120");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 9);

        let hex = &String::from("D8005AC2A8F0");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 1);

        let hex = &String::from("F600BC2D8F");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 0);

        let hex = &String::from("9C005AC2F8F0");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 0);

        let hex = &String::from("9C0141080250320F1802104A08");
        let mut bstrm = bitstream(&hex);
        let pkt:Box<Packet> = parse_packet(&mut bstrm);
        assert_eq!(value_of(&pkt), 1);
    }
}
