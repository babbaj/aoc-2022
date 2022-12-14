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

fn compare_slice(left: &[Packet], right: &[Packet]) -> Ordering {
    let len = min(left.len(), right.len());
    for i in 0..len {
        let order = compare(&left[i], &right[i]);
        if order != Ordering::Equal {
            return order;
        }
    }

    return left.len().cmp(&right.len());
}

fn compare(first: &Packet, second: &Packet) -> Ordering {
    match (first, second) {
        (Num(left), Num(right)) => left.cmp(right),
        (List(left), List(right)) => compare_slice(left, right),
        (List(left), Num(right)) => compare_slice(left, &[Num(*right)]),
        (Num(left), List(right)) => compare_slice(&[Num(*left)], right)
    }
}

fn main() {
    let input = include_str!("../input");

    let packets = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_packet(line).unwrap().1)
        .collect::<Vec<_>>();

    let pairs = (0..packets.len()).step_by(2)
        .map(|i| (&packets[i], &packets[i + 1]));
    let part1 = pairs.enumerate()
        .map(|(i, (a, b))| (i, compare(a, b)))
        .filter(|(_, ord)| *ord != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let dividers = [2, 6].map(|n|
        List(vec![List(vec![Num(n)])])
    );

    let part2: i32 = packets.iter()
        .map(|packet| [compare(&dividers[0], packet), compare(&dividers[1], packet)])
        .map(|ord| ord.map(|o| match o {
            Ordering::Greater => 1,
            _ => 0
        }))
        .fold([1, 2], |acc, ord| [acc[0] + ord[0], acc[1] + ord[1]])
        .iter().product();

    println!("part 1 = {}, part 2 = {}", part1, part2);
}
