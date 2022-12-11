use nom::{IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, line_ending};
use nom::combinator::{map, map_res, recognize, value};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, preceded, pair, terminated, tuple};
use crate::Operation::{AddConst, MulConst};

#[derive(Clone, Copy)]
enum Operation {
    MulOld,
    AddConst(i64),
    MulConst(i64)
}

#[derive(Clone, Copy)]
struct Test {
    divisible_by: i32,
    throw_to: (i32, i32) // (true, false)
}

struct Monkey {
    starting_items: Vec<i64>,
    operation: Operation,
    test: Test
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |x| i32::from_str_radix(x, 10))(input)
}
fn parse_64(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |x| i64::from_str_radix(x, 10))(input)
}

fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, _) = tag("old ")(i)?;
    let (i, op) = alt((
        value(Operation::MulOld, tag("* old")),
        map(separated_pair(alt((char('+'), char('*'))), char(' '), parse_64), |(op, num)| match op {
            '+' => AddConst(num),
            '*' => MulConst(num),
            _ => unreachable!()
        })
    ))(i)?;
    Ok((i, op))
}

fn parse_throw_to(i: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("    If true: throw to monkey "), parse_num),
        line_ending,
        preceded(tag("    If false: throw to monkey "), parse_num)
    )(i)
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = recognize(tuple((tag("Monkey "), digit1, char(':'), line_ending)))(i)?;
    let (i, items) = terminated(preceded(tag("  Starting items: "), separated_list1(tag(", "), parse_64)), line_ending)(i)?;
    let (i, op) = terminated(preceded(tag("  Operation: new = "), parse_operation), line_ending)(i)?;
    let (i, div_by) = terminated(preceded(tag("  Test: divisible by "), parse_num), line_ending)(i)?;
    let (i, throw_to) = parse_throw_to(i)?;
    let monkey = Monkey {
        starting_items: items,
        operation: op,
        test: Test {
            divisible_by: div_by,
            throw_to
        }
    };
    Ok((i, monkey))
}

fn parse_input(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(pair(line_ending, line_ending),parse_monkey)(i)
}

fn do_test(test: Test, worry: i64) -> i32 {
    if worry % (test.divisible_by as i64) == 0 {
        test.throw_to.0
    } else {
        test.throw_to.1
    }
}

fn apply_op(op: Operation, item: i64) -> i64 {
    match op {
        Operation::MulOld => item * item,
        MulConst(x) => item * x,
        AddConst(x) => item + x
    }
}

fn do_round(monkeys: &mut Vec<(i64, Monkey)>) {
    for i in 0..monkeys.len() {
        let (inspections, monk): &mut (i64, Monkey) = unsafe { &mut *(&mut monkeys[i] as *mut _) };

        monk.starting_items.drain(..).for_each(|item| {
            let mut worry = apply_op(monk.operation, item);
            //worry /= 3;
            //worry = worry % 96577;
            worry = worry % 9699690; // LCM
            let throw_to = do_test(monk.test, worry);
            assert_ne!(i as i32, throw_to);
            monkeys[throw_to as usize].1.starting_items.push(worry);
            *inspections += 1;
        })
    }
}

fn main() {
    let input = include_str!("../input");
    let (rest, monkeys_parsed) = parse_input(input).unwrap();
    assert!(rest.is_empty());

    let mut monkeys = monkeys_parsed.into_iter().map(|m| (0, m)).collect();
    for r in 0..10000 {
        do_round(&mut monkeys);
    }
    let mut sorted_inspections = monkeys.iter().map(|(inspections, _)| *inspections).collect::<Vec<_>>();
    sorted_inspections.sort_by(|a, b| b.cmp(a)); // desc
    println!("monkey business = {}", sorted_inspections[0] * sorted_inspections[1]);

}
