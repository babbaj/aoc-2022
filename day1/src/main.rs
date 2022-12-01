fn main() {
    let input = include_str!("../input");
    let elves: Vec<Vec<i32>> = input.split("\n\n")
        .map(|lines|
            lines.split("\n")
            .map(|l| l.parse().unwrap())
            .collect()
        )
        .collect();
    let mut sums: Vec<i32> = elves.iter()
        .map(|elf| elf.iter().sum())
        .collect();
    sums.sort();

    println!("part 1 = {}", sums.last().unwrap());
    println!("part 2 = {}", sums.iter().rev().take(3).sum::<i32>())
}
