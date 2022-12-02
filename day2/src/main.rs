fn main() {
    let input = include_str!("../input");

    let strategy = input.split("\n")
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()));

    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSOR: i32 = 3;
    const LOSE: i32 = 0;
    const TIE: i32 = 3;
    const WIN: i32 = 6;
    let mut scorePart1: i32 = 0;
    let mut scorePart2: i32 = 0;
    for round in strategy {
        match round {
            ('A', 'X') => {
                scorePart1 += ROCK + TIE;
                scorePart2 += SCISSOR + LOSE;
            },
            ('A', 'Y') => {
                scorePart1 += PAPER + WIN;
                scorePart2 += ROCK + TIE;
            },
            ('A', 'Z') => {
                scorePart1 += SCISSOR + LOSE;
                scorePart2 += PAPER + WIN;
            },
            ('B', 'X') => {
                scorePart1 += ROCK + LOSE;
                scorePart2 += ROCK + LOSE;
            }
            ('B', 'Y') => {
                scorePart1 += PAPER + TIE;
                scorePart2 += PAPER + TIE;
            }
            ('B', 'Z') => {
                scorePart1 += SCISSOR + WIN;
                scorePart2 += SCISSOR + WIN;
            },
            ('C', 'X') => {
                scorePart1 += ROCK + WIN;
                scorePart2 += PAPER + LOSE;
            }
            ('C', 'Y') => {
                scorePart1 += PAPER + LOSE;
                scorePart2 += SCISSOR + TIE;
            }
            ('C', 'Z') => {
                scorePart1 += SCISSOR + TIE;
                scorePart2 += ROCK + WIN;
            },
            _ => panic!()
        }
    }

    println!("part 1 = {}", scorePart1);
    println!("part 2 = {}", scorePart2);
}
