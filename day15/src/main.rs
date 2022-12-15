use std::collections::HashSet;
use std::ops::Range;

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

fn main() {
    let input = include_str!("../input");

    let nodes = input.lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    const ROW: i32 = 2000000;
    //const ROW: i32 = 10;

    let mut sum = 0;
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

    println!("{}", set.len());
}
