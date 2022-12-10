use std::{mem, str};

enum Instruction {
    Noop,
    Addx(i32)
}

fn main() {
    let input = include_str!("../input");
    let instructions = input.lines()
        .map(|line| match &line[0..4] {
            "noop" => Instruction::Noop,
            "addx" => {
                let (_, num) = line.split_once(' ').unwrap();
                Instruction::Addx(num.parse().unwrap())
            },
            _ => unreachable!()
        });

    let mut x = 1;
    let mut values = Vec::new();

    for inst in instructions {
        match inst {
            Instruction::Noop => {
                values.push(x);
            },
            Instruction::Addx(num) => {
                values.push(x);
                values.push(x);
                x += num;
            }
        }
    }

    let with_cycles = || values.iter().copied()
        .enumerate()
        .map(|(i, x)| ((i + 1) as i32, x));

    let sum: i32 = with_cycles()
        .skip(19)
        .step_by(40)
        .take(6)
        .map(|(i, x)| x * i)
        .sum();
    println!("part 1 = {}", sum);


    let mut crt = [[b' '; 40]; 6];
    for (cycle, x) in with_cycles() {
        let index = cycle - 1;
        let row = index / 40;
        let col = index % 40;
        if [x - 1, x, x + 1].contains(&col) {
            crt[row as usize][col as usize] = b'#';
        }
    }
    render_crt(&crt);
}

fn render_crt(crt: &[[u8; 40]; 6]) {
    for row in crt {
        let s = unsafe { str::from_utf8_unchecked(row) };
        println!("{s}");
    }
}