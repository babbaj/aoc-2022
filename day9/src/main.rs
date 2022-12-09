use std::collections::HashSet;

type Pos = (i32, i32);

fn main() {
    let input = include_str!("../input");
    let moves = input.lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(dir, num)| (dir.as_bytes()[0], num.parse::<u8>().unwrap()));

    let mut knots = [(0, 0); 10];
    let mut part1_visits = HashSet::<Pos>::new();
    let mut part2_visits = HashSet::<Pos>::new();
    part1_visits.insert((0, 0));
    part2_visits.insert((0, 0));

    for (dir, dist) in moves {
        for _ in 0..dist {
            knots[0] = move_head(dir, knots[0]);
            for i in 1..=9 {
                let h = knots[i - 1];
                let t = knots[i];
                if let Some(new_tail) =  follow_knot(h, t) {
                    knots[i] = new_tail;
                }
            }
            part1_visits.insert(knots[1]);
            part2_visits.insert(knots[9]);
        }
    }

    println!("part 1 = {}", part1_visits.len());
    println!("part 2 = {}", part2_visits.len());
}

fn move_head(dir: u8, (x, y): Pos) -> Pos {
    match dir {
        b'U' => (x, y + 1),
        b'D' => (x, y - 1),
        b'L' => (x - 1, y),
        b'R' => (x + 1, y),
        _ => panic!()
    }
}

fn sub((ax, ay): Pos, (bx, by): Pos) -> (i32, i32) {
    (ax - bx, ay - by)
}

fn touching(h: Pos, t: Pos) -> bool {
    let (dx, dy) = sub(h, t);
    dx.abs() < 2 && dy.abs() < 2
}

fn dist(h: Pos, t: Pos) -> i32 {
    let (dx, dy) = sub(h, t);
    dx.abs() + dy.abs()
}

fn follow_knot(head: Pos, (tx, ty): Pos) -> Option<Pos> {
    if touching(head, (tx, ty)) {
        return None // already touching, no new move
    }

    [
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
        .min_by_key(|new_tail| dist(*new_tail, head))
}
