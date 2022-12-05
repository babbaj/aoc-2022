#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize
}

fn parse_move(line: &str) -> Move {
    let mut iter = line.split(' ');
    iter.next().unwrap(); // "move "
    let quantity = iter.next().unwrap().parse().unwrap();
    iter.next().unwrap(); // " from "
    let from = iter.next().unwrap().parse().unwrap();
    iter.next().unwrap(); // " to "
    let to = iter.next().unwrap().parse().unwrap();
    return Move {
        quantity,
        from,
        to
    }
}
fn main() {
    let input = include_str!("../input");
    let input_lines = input.lines().collect::<Vec<_>>();

    let number_line_idx = input_lines.iter()
        .position(| s| s.starts_with(" 1"))
        .unwrap();
    let drawing = &input_lines[..number_line_idx];
    let drawing_lines: Vec<String> = drawing.iter()
        .map(|s| &s[1..s.len() - 1])
        .map(|line| line.chars().step_by(4).collect::<String>())
        .collect();

    let num_cols = drawing_lines.iter()
        .map(|s| s.len())
        .max().unwrap();

    let mut stacks: Vec<Vec<char>> = (0..num_cols).map(|_| Vec::new()).collect();
    for str in drawing_lines.iter().rev() {
        for (idx, c) in str.char_indices().filter(|(_, c)| *c != ' ') {
            stacks[idx].push(c)
        }
    }
    let moves_lines = &input_lines[number_line_idx + 2..];
    for l in moves_lines {
        let mov = parse_move(l);
        //do_move_single(&mut stacks, &mov);
        do_move_multi(&mut stacks, &mov);
    }

    stacks.iter().map(|v| v.iter().collect::<String>()).for_each(|column| println!("{column}"));
    let top = stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
    println!("top = {top}");
}

fn do_move_single(stacks: &mut Vec<Vec<char>>, mov: &Move) {
    // could be optimized by doing the same thing as do_move_multi but with reversed source slice
    for _ in 0..mov.quantity {
        let popped = stacks[mov.from - 1].pop().unwrap();
        stacks[mov.to - 1].push(popped);
    }
}

fn do_move_multi(stacks: &mut Vec<Vec<char>>, mov: &Move) {
    let from_stack: &mut Vec<char> = unsafe { &mut *(&mut stacks[mov.from - 1] as *mut _) };
    let start_idx = from_stack.len() - mov.quantity;
    let source = &from_stack[start_idx..];

    let dest = &mut stacks[mov.to - 1];
    dest.extend_from_slice(source);
    from_stack.truncate(start_idx);
}

