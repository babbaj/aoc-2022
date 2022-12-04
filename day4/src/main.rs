use nom::{IResult};
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res};
use nom::sequence::{separated_pair};

fn main() {
    let input = include_str!("../input");

    let data: Vec<((i32, i32), (i32, i32))> = input.lines()
        //.map(|line| line.split_once(',').unwrap())
        //.map(|(a, b)| (a.split_once('-').unwrap(), b.split_once('-').unwrap()))
        //.map(|((a1, b1), (a2, b2))| ((a1.parse().unwrap(), b1.parse().unwrap()), (a2.parse().unwrap(), b2.parse().unwrap())))
        .map(parse_line)
        .collect();

    let part1 = data.iter()
        .filter(|(elf1, elf2)| range_fully_contains(elf1, elf2) || range_fully_contains(elf2, elf1))
        .count();
    println!("part 1 = {}", part1);
    let part2 = data.iter()
        .filter(|(elf1, elf2)| ranges_overlap(elf1, elf2))
        .count();
    println!("part 2 = {}", part2);
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |x| i32::from_str_radix(x, 10))(input)
}

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let parse_range = |input| separated_pair(parse_num, char('-'), parse_num)(input);
    separated_pair(parse_range, char(','), parse_range)(line).unwrap().1
}

fn range_fully_contains(outer: &(i32, i32), inner: &(i32, i32)) -> bool {
    outer.0 <= inner.0 && outer.1 >= inner.1
}

fn ranges_overlap(a: &(i32, i32), b: &(i32, i32)) -> bool {
    // check if a overlaps with b
    (a.0 >= b.0 && a.0 <= b.1)
    || (a.1 >= b.0 && a.1 <= b.1)
    // check if b overlaps with a
    || (b.0 >= a.0 && b.0 <= a.1)
    || (b.1 >= a.0 && b.1 <= a.1)
}