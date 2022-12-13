use std::cmp::{min, Ordering};
use nom::IResult;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use crate::Packet::*;

#[derive(Debug, Clone, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Num(i32)
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |x| i32::from_str_radix(x, 10))(input)
}

fn parse_packet(packet: &str) -> IResult<&str, Packet> {
    alt((
        map(delimited(
            char('[', ),
            separated_list0(char(','), parse_packet),
            char(']')
        ), |v| List(v)),
        map(parse_num, |n| Num(n))
    ))(packet)
}

fn compare(first: &Packet, second: &Packet) -> Ordering {
    match (first, second) {
        (Num(n1), Num(n2)) => n1.cmp(n2),
        (List(l1), List(l2)) => {
            let len = min(l1.len(), l2.len());
            for i in 0..len {
                let order = compare(&l1[i], &l2[i]);
                if order != Ordering::Equal {
                    return order;
                }
            }

            return l1.len().cmp(&l2.len());
        },
        (List(_), Num(n2)) => compare(first, &List(vec![Num(*n2)])),
        (Num(n1), List(_)) => compare(&List(vec![Num(*n1)]), second)
    }
}

fn main() {
    let input = include_str!("../input");

    let mut packets = input.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_packet(line).unwrap().1)
        .collect::<Vec<_>>();

    let pairs = (0..packets.len()).step_by(2)
        .map(|i| (&packets[i], &packets[i + 1]));
    let part1 = pairs.enumerate()
        .map(|(i, (a, b))| (i, compare(&a, &b)))
        .filter(|(_, ord)| *ord != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let dividers = [2, 6].map(|n|
        List(vec![List(vec![Num(n)])])
    );
    packets.extend_from_slice(&dividers);
    packets.sort_by(compare);

    let part2: usize = packets.iter().enumerate()
        .filter(|(_, packet)| dividers.contains(packet))
        .map(|(i, _)| i + 1)
        .product();

    println!("part 1 = {}, part 2 = {}", part1, part2);
}
