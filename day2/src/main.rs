fn main() {
    let input = include_str!("../input");

    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSOR: i32 = 3;
    const LOSE: i32 = 0;
    const TIE: i32 = 3;
    const WIN: i32 = 6;
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    for round in input.split("\n") {
        match round {
            "A X" => {
                part1 += ROCK + TIE;
                part2 += SCISSOR + LOSE;
            },
            "A Y" => {
                part1 += PAPER + WIN;
                part2 += ROCK + TIE;
            },
            "A Z" => {
                part1 += SCISSOR + LOSE;
                part2 += PAPER + WIN;
            },
            "B X" => {
                part1 += ROCK + LOSE;
                part2 += ROCK + LOSE;
            }
            "B Y" => {
                part1 += PAPER + TIE;
                part2 += PAPER + TIE;
            }
            "B Z" => {
                part1 += SCISSOR + WIN;
                part2 += SCISSOR + WIN;
            },
            "C X" => {
                part1 += ROCK + WIN;
                part2 += PAPER + LOSE;
            }
            "C Y" => {
                part1 += PAPER + LOSE;
                part2 += SCISSOR + TIE;
            }
            "C Z" => {
                part1 += SCISSOR + TIE;
                part2 += ROCK + WIN;
            },
            _ => panic!()
        }
    }

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
