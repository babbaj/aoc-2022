use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Cell {
    Start,
    End,
    H(u8)
}

fn height(cell: Cell) -> u8 {
    match cell {
        Cell::Start => 0,
        Cell::End => b'z' - b'a',
        Cell::H(h) => h
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cell: Cell
}

type Map = Vec<Vec<Cell>>;

fn neighbors<'a>(node: Node, map: &'a Map, ascending: bool) -> impl Iterator<Item = Node> + 'a
{
    let size_x = map[0].len() as isize;
    let size_y = map.len() as isize;
    [(0, 1), (1, 0), (0, -1), (-1, 0)] // up, right, down, left
        .map(|(dx, dy)| (node.x as isize + dx, node.y as isize + dy))
        .into_iter()
        .filter(move |(x, y)| (0..size_x).contains(&x) && (0..size_y).contains(&y))
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| Node {x, y, cell: map[y][x]})
        .filter(move |neighbor|
            if ascending { height(neighbor.cell) <= height(node.cell) + 1 }
            else { height(neighbor.cell) + 1 >= height(node.cell) }
        )
}


fn bfs_solve(graph: &Map, start: Node) -> Option<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0usize, start));
    let mut closest = None;
    let mut start = None;
    while !queue.is_empty() {
        let (depth, node) = queue.pop_front().unwrap();
        if start.is_none() && node.cell == Cell::Start {
            start = Some(depth);
        }
        if closest.is_none() && node.cell == Cell::H(0u8) {
            closest = Some(depth);
        }
        if start.is_some() && closest.is_some() {
            return Some((start.unwrap(), closest.unwrap()));
        }

        neighbors(node, graph, false)
            .filter(|n| visited.insert(*n))
            .for_each(|neighbor| queue.push_back((depth + 1, neighbor)));
    }
    None
}


fn main() {
    let input = include_bytes!("../bigboy.txt");
    let map: Map = input.split(|c| *c == b'\n')
        .filter(|l| l.len() > 0)
        .map(|line| line.iter().map(|c| match c {
            b'S' => Cell::Start,
            b'E' => Cell::End,
            c => Cell::H(c - b'a')
        }).collect())
        .collect();

    let mut end = None;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let cell = map[y][x];
            if cell == Cell::End {
                end = Some(Node { x, y, cell });
                break;
            }
        }
    };

    let (part1, part2) = bfs_solve(&map, end.unwrap()).unwrap();
    println!("part 1 = {}, part 2 = {}", part1, part2);
}
