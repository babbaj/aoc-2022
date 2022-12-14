use std::cmp;
use crate::Cell::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Cell {
    Air,
    Rock,
    Sand
}

fn parse_pos(s: &str) -> Pos {
    let (a, b) = s.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

type Pos = (i32, i32);

fn down((x, y): Pos) -> Pos {
    (x, y + 1)
}
fn left((x, y): Pos) -> Pos {
    (x - 1, y)
}
fn right((x, y): Pos) -> Pos {
    (x + 1, y)
}

fn get(grid: &Vec<Vec<Cell>>, (x, y): Pos) -> Cell {
    *grid.get(y as usize).and_then(|v| v.get(x as usize)).unwrap_or(&Air)
}

fn do_line<F: FnMut(i32, i32)>((x1, y1): Pos, (x2, y2): Pos, mut f: F) {
    let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());
    let (mut x, mut y) = (x1, y1);
    while (x, y) != (x2, y2) {
        f(x, y);
        (x, y) = (x + dx, y + dy);
    }
    f(x, y);
}

fn populate_rocks(grid: &mut Vec<Vec<Cell>>, rocks: &Vec<Vec<Pos>>, min_x: i32) {
    for v in rocks {
        v.windows(2)
            .for_each(|w| do_line(w[0], w[1], |x, y| {
                grid[y as usize]
                    [( 1000 + (x - min_x)) as usize] = Rock;
            }))
    }
}

// returns None if at rest
fn next_pos(grid: &Vec<Vec<Cell>>, sand: Pos) -> Option<Pos> {
    if sand.1 == (grid.len() - 2) as i32 {
        return None
    }
    if get(grid, down(sand)) == Air {
        return Some(down(sand))
    }
    if get(grid, down(left(sand))) == Air {
        return Some(down(left(sand)))
    }
    if get(grid, down(right(sand))) == Air {
        return Some(down(right(sand)))
    }
    None
}

// returns None if the sand has nowhere to rest
// part 2: returns None if clogged
fn simulate_sand(grid: &Vec<Vec<Cell>>, start_x: i32) -> Option<Pos> {
    let max_y = (grid.len() - 1) as i32;
    let max_x = (grid[0].len() - 1) as i32;
    let mut sand = (start_x, 0);
    /*while let Some((x, y)) = next_pos(grid, sand) {
        if x < 0 || x > max_x {
            return None
        }
        if y >= max_y {
            return None
        }
        sand = (x, y);
    }*/
    while let Some((x, y)) = next_pos(grid, sand) {
        if y == 0 && grid[0usize][start_x as usize] == Sand {
            return None
        }
        sand = (x, y)

    }
    Some(sand)
}

fn do_simulation(grid: &mut Vec<Vec<Cell>>, min_x: i32) {
    let mut at_rest = 0;
    let start_x = (500 - min_x) + 1000;
    while let Some((x, y)) = simulate_sand(grid, start_x) {
        assert_eq!(grid[y as usize][x as usize], Air);
        grid[y as usize][x as usize] = Sand;
        at_rest += 1;
    }
    println!("at_rest = {}", at_rest);
}

fn main() {
    let input = include_str!("../input");

    let rocks: Vec<Vec<Pos>> = input.lines()
        .map(|line| line.split(" -> ").map(parse_pos).collect())
        .collect();
    let (min_x, (max_x, max_y)) = rocks.iter()
        .flatten()
        .copied()
        .fold((rocks[0][0].0, rocks[0][0]), |(min_x, (max_x, max_y)), (x, y)| {
            (cmp::min(min_x, x), (cmp::max(max_x, x), cmp::max(max_y, y)))
        });
    let (width, height) = ((max_x - min_x) + 1, max_y + 1);
    let width = 2000;
    let height = height + 2;
    let mut grid = vec![vec![Air; width as usize]; height as usize];
    populate_rocks(&mut grid, &rocks, min_x);

    do_simulation(&mut grid, min_x);


    println!("Hello, world!");
}
