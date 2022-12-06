use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let index_of = input.as_bytes().windows(4).position(is_marker).unwrap();
    println!("part 1 = {}", index_of + 4);

    let index_of = input.as_bytes().windows(14).position(is_marker).unwrap();
    println!("part 2 = {}", index_of + 14);
}

fn is_marker(s: &[u8]) -> bool {
    return HashSet::<u8>::from_iter(s.iter().copied()).len() == s.len()
}
