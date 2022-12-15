use std::collections::HashSet;

type Pos = (i32, i32);

fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
    let line = &line["Sensor at x=".len()..];
    let i = line.find(',').unwrap();
    let sensor_x = &line[..i];
    let line = &line[i + ", y=".len()..];
    let i = line.find(':').unwrap();
    let sensor_y = &line[..i];
    let line = &line[i + ": closest beacon is at x=".len()..];
    let (beacon_x, beacon_y) = line.split_once(", y=").unwrap();

    ((sensor_x.parse().unwrap(), sensor_y.parse().unwrap()), (beacon_x.parse().unwrap(), beacon_y.parse().unwrap()))
}

fn dist((x1, y1): Pos, (x2, y2): Pos) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn row_at_offset(scanner_x: i32, dist_to_beacon: i32, y_offset: i32) -> Option<(i32, i32)> {
    //let width0 = (dist_to_beacon * 2) + 1; // widest width
    // every step down the width decreases by 2
    //let width = width0 - (y_offset * 2);
    if y_offset > dist_to_beacon {
        return None
    }
    let x_offset = dist_to_beacon - y_offset;
    Some(((scanner_x - x_offset), (scanner_x + x_offset)))
}


// size = dist to beacon
fn scan_boundary<F: Fn(i32, i32)>((sex, sy): Pos, size: i32, f: F) {
    let points = [(0, 1), (1, 0), (0, -1), (-1, 0)] // up, right, down, left
        .map(|(dx, dy)| ((sex + dx * (size + 1)), (sy + dy * (size + 1))));
    let mut iter = points.iter().cycle().peekable();
    for _ in 0..4 {
        let (mut x, mut y) = iter.next().unwrap();
        let (next_x, next_y) = iter.peek().unwrap();
        let (dx, dy) = ((next_x - x).signum(), (next_y - y).signum());
        while (x, y) != (*next_x, *next_y) {
            f(x, y);
            (x, y) = (x + dx, y + dy);
        }
    }
}

fn in_scanner_range(point: Pos, scanner: Pos, range: i32) -> bool {
    dist(point, scanner) <=  range
}

fn valid(x: i32, y: i32) -> bool {
    let max = 4000000;
    //let max = 20;
    x > 0 && x <= max && y > 0 && y <= max
}
fn tuning_frequency(x: i32, y: i32) -> i64 {
    (x as i64) * 4000000 + (y as i64)
}

fn main() {
    let input = include_str!("../input");

    let nodes = input.lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    //part1(&nodes);

    for ((sex, sy), (bx, by)) in nodes.iter().copied() {
        let d = dist((sex, sy), (bx, by));
        scan_boundary((sex, sy), d, |x, y| {
            if !nodes.iter().copied().any(|((sex, sy), (bx, by))| {
                let range = dist((sex, sy), (bx, by));
                in_scanner_range((x, y), (sex, sy), range)
            })
            {
                if valid(x, y) {
                    println!("{:?}", (x, y));
                    println!("tuning = {}", tuning_frequency(x, y));
                    std::process::exit(0); // lol
                }
            }
        })
    }
}

fn part1(nodes: &Vec<(Pos, Pos)>) {
    const ROW: i32 = 2000000;

    let ranges = nodes.iter().copied()
        .filter_map(|((sex, sy), beacon)| {
            let dist = dist((sex, sy), beacon);
            let offset_from_row = (ROW - sy).abs();
            row_at_offset(sex, dist, offset_from_row)
        })
        .collect::<Vec<_>>();
    let mut set = HashSet::<i32>::new();
    for (x1, x2) in ranges.iter().copied() {
        for x in x1..=x2 {
            set.insert(x);
        }
    }
    for (_, (bx, by)) in nodes.iter() {
        if *by == ROW {
            set.remove(bx);
        }
    }

    println!("part 1 = {}", set.len());
}
