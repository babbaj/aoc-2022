use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    part1(input);
    part2(input);
}

fn part2(input: &str) {
    let mut lines = input.lines();

    let mut sum: i32 = 0;
    loop {
        //let Some(a) = lines.next() else { break };
        //let Some(b) = lines.next() else { break };
        //let Some(c) = lines.next() else { break };
        let a = lines.next();
        if a.is_none() { break }
        let a = a.unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();
        let setA = HashSet::<char>::from_iter(a.chars());
        let setB = HashSet::<char>::from_iter(b.chars());
        let setC = HashSet::<char>::from_iter(c.chars());
        let intersection = setA.into_iter()
            .filter(|c| setB.contains(c) && setC.contains(c))
            .next()
            .unwrap();

        sum += priority(intersection);
    }
    println!("part 2 = {}", sum);
}

fn part1(input: &str) {
    let compartments = input.lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| (HashSet::<char>::from_iter(a.chars()), HashSet::<char>::from_iter(b.chars())));

    let mut sum: i32 = 0;
    for (a, b) in compartments {
        let intersect = a.intersection(&b);
        sum += intersect.map(|c| priority(*c)).sum::<i32>();
    }
    println!("part 1 = {}", sum);
}

fn priority(c: char) -> i32 {
    let num = c as u8;
    (match num {
        b'A'..=b'Z' => (num - b'A') + 27,
        b'a'..=b'z' => (num - b'a') + 1,
        _ => panic!()
    }) as i32
}