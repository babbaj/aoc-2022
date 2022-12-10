use std::str;

enum Instruction {
    Noop,
    Addx(i32)
}

fn parse_line(line: &[u8]) -> Instruction {
    let instruction = &line[0..4];
    if instruction == b"noop" {
        return Instruction::Noop;
    }
    if instruction == b"addx" {
        let num = unsafe { str::from_utf8_unchecked(&line[5..]) };
        return Instruction::Addx(num.parse().unwrap());
    }
    panic!("bad input");
}

fn main() {
    let input = include_bytes!("../input");
    let instructions = input.split(|b| *b == b'\n')
        .map(parse_line);

    let values = instructions
        .scan((1, 1), |(cycle, x), inst| {
            let val = *x;
            let n = match inst {
                Instruction::Noop => 1,
                Instruction::Addx(num) => {
                    *x += num;
                    2
                }
            };
            *cycle += 1;
            Some(std::iter::repeat(val).take(n))
        })
        .flatten()
        .enumerate()
        .map(|(i, x) | (i as i32 + 1, x));

    let mut part1 = 0;
    let mut crt = [[b' '; 40]; 6];
    values.for_each(| (cycle, x)| {
        if (cycle - 20) % 40 == 0 && cycle <= 220 { part1 += x * cycle };

        let index = cycle - 1;
        let row = index / 40;
        let col = index % 40;
        if [x - 1, x, x + 1].contains(&col) {
            crt[row as usize][col as usize] = b'#';
        }
    });

    println!("part 1 = {}", part1);
    render_crt(&crt);
}

fn render_crt(crt: &[[u8; 40]; 6]) {
    for row in crt {
        let s = unsafe { str::from_utf8_unchecked(row) };
        println!("{s}");
    }
}