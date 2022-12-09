use std::collections::HashSet;

type Pos = (i32, i32);
type Move = (char, u8);
#[derive(Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R
}
impl Dir {
    fn parse(s: &str) -> Dir {
        match s {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!()
        }
    }
}


fn main() {
    let input = include_str!("../input");
    let moves = input.lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, num)| (Dir::parse(a), num.parse::<u8>().unwrap()));
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_visits = HashSet::<(i32, i32)>::new();
    tail_visits.insert(tail);

    for (dir, dist) in moves {
        for _ in 0..dist {
            let new_head = move_head(dir, head);
            match move_to_head(new_head, tail) {
                Some(new_tail) => {
                    tail_visits.insert(new_tail);
                    tail = new_tail;
                },
                None => {}
            }
            head = new_head;
        }
    }

    println!("part 1 = {}", tail_visits.len());
}

fn move_head(dir: Dir, (x, y): Pos) -> Pos {
    match dir {
        Dir::U => (x, y + 1),
        Dir::D => (x, y - 1),
        Dir::L => (x - 1, y),
        Dir::R => (x + 1, y)
    }
}

fn sub((ax, ay): Pos, (bx, by): Pos) -> (i32, i32) {
    (ax - bx, ay - by)
}

fn touching(h: Pos, t: Pos) -> bool {
    let (dx, dy) = sub(h, t);
    dx.abs() < 2 && dy.abs() < 2
}

fn adjacent(h: Pos, t: Pos) -> bool {
    let (dx, dy) = sub(h, t);
    dx.abs() + dy.abs() == 1
}

fn move_to_head(head: Pos, (tx, ty): Pos) -> Option<Pos> {
    if touching(head, (tx, ty)) {
        return None // touching
    }

    let mut new_tails = [
        (-1, 1), // up left
        (0, 1), // up
        (1, 1), // up right
        (-1, 0), // left
        (1, 0), // right
        (-1, -1), // down left
        (0, -1), // down
        (1, -1) // down right
    ].iter()
        .map(|(dx, dy)| (tx + dx, ty + dy))
        .filter(|new_tail| adjacent(head, *new_tail));

    let new_tail = new_tails.next().unwrap();
    assert_eq!(new_tails.next(), None);
    Some(new_tail)
}
