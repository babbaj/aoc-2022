use std::collections::{HashMap, HashSet};

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

fn neighbors<'a>(node: Node, map: &'a Map) -> impl Iterator<Item = Node> + 'a
{
    let size_x = map[0].len() as isize;
    let size_y = map.len() as isize;
    [(0, 1), (1, 0), (0, -1), (-1, 0)] // up, right, down, left
        .map(|(dx, dy)| (node.x as isize + dx, node.y as isize + dy))
        .into_iter()
        .filter(move |(x, y)| (0..size_x).contains(&x) && (0..size_y).contains(&y))
        .map(|(x, y)| (x as usize, y as usize))
        .map(|(x, y)| Node {x, y, cell: map[y][x]})
        .filter(move |neighbor| height(neighbor.cell) <= height(node.cell) + 1)
}


fn dijkstras0(graph: &Map, start: Node) -> Option<Vec<Vec<Option<Node>>>> {
    let mut dist = vec![vec![i32::MAX; graph[0].len()]; graph.len()];
    let mut prev = vec![vec![Option::<Node>::None; graph[0].len()]; graph.len()];

    dist[start.y][start.x] = 0;
    let mut unvisited = HashSet::<Node>::new();
    for y in 0..graph.len() {
        for x in 0..graph[0].len() {
            unvisited.insert(Node { x, y, cell: graph[y][x]});
        }
    }

    while !unvisited.is_empty() {
        let current = *unvisited.iter().min_by_key(|node| dist[node.y][node.x]).unwrap();
        if dist[current.y][current.x] == i32::MAX {
            return None;
        }
        if current.cell == Cell::End {
            break;
        }

        unvisited.remove(&current);

        neighbors(current, graph).filter(|neighbor| unvisited.contains(neighbor))
            .for_each(|neighbor| {
                let alt = dist[current.y][current.x] + 1;
                if alt <= dist[neighbor.y][neighbor.x] {
                    dist[neighbor.y][neighbor.x] = alt;
                    prev[neighbor.y][neighbor.x] = Some(current);
                }
            });
    }
    Some(prev)
}

fn dijkstras(graph: &Map, start: Node, target: Node) -> Option<Vec<Node>> {
    assert_eq!(target.cell, Cell::End);
    match dijkstras0(graph, start) {
        Some(prev) => {
            let mut path = Vec::new();
            let mut node = target;
            if prev[node.y][node.x].is_some() {
                while let Some(prev_node) = prev[node.y][node.x] {
                    path.push(node);
                    node = prev_node;
                }
            }
            Some(path)
        },
        None => None
    }
}


fn main() {
    let input = include_bytes!("../input");
    let map: Map = input.split(|c| *c == b'\n')
        .map(|line| line.iter().map(|c| match c {
            b'S' => Cell::Start,
            b'E' => Cell::End,
            c => Cell::H(c - b'a')
        }).collect())
        .collect();

    // lazy lol
    //let mut start = Node { x: 0, y: 0, cell: Cell::Start };
    let mut end = Node { x: 0, y: 0, cell: Cell::End };
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let cell = map[y][x];
            /*if cell == Cell::Start {
                start.x = x as i32;
                start.y = y as i32;
            }*/
            if cell == Cell::End {
                end.x = x;
                end.y = y;
            }
        }
    }
    //let path = dijkstras(&map, start, end);
    let min = (0..map[0].len()).flat_map(|x| (0..map.len()).map(move |y| (x, y)))
        .filter(|(x, y)| map[*y][*x] == Cell::H(0))
        .filter_map(|(x, y)| dijkstras(&map, Node {x, y, cell: Cell::H(0)}, end))
        .map(|path| path.len())
        .min();
    println!("{}", min.unwrap())

    //println!("path size = {}", path.len());
}
