use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Cell {
    Start,
    End,
    H(u8)
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct Node {
    x: i32,
    y: i32,
    cell: Cell
}

type Map = Vec<Vec<Cell>>;

const END_HEIGHT: u8 = (b'z' - b'a');
const START_HEIGHT: u8 = 0;

fn neighbors<'a>(node: Node, map: &'a Map) -> impl Iterator<Item = Node> + 'a
{
    let size_x = map[0].len() as i32;
    let size_y = map.len() as i32;
    [(0, 1), (1, 0), (0, -1), (-1, 0)] // up, right, down, left
        .map(|(dx, dy)| (node.x + dx, node.y + dy))
        .into_iter()
        .filter(move |(x, y)| (0..size_x).contains(&x) && (0..size_y).contains(&y))
        .map(|(x, y)| Node {x, y, cell: map[y as usize][x as usize]})
        .filter(move |neighbor| match (node.cell, neighbor.cell) {
            (Cell::H(node_height), Cell::H(neighbor_height)) => neighbor_height <= node_height + 1,
            (Cell::H(node_height), Cell::End) => node_height >= END_HEIGHT - 1,
            (Cell::H(_), Cell::Start) => true,
            (Cell::Start, Cell::H(neighbor_height)) => neighbor_height <= START_HEIGHT + 1,
            (Cell::Start, Cell::End) => unreachable!("lol"),
            (Cell::End, _) => unreachable!("end node trying to check neighbors?"),
            (_, Cell::Start) => unreachable!("trying to visit start node")
        })
}



fn dijkstras0(graph: &Map, start: Node) -> (Vec<Vec<i32>>, Vec<Vec<Option<Node>>>) {
    let mut dist = vec![vec![i32::MAX; graph[0].len()]; graph.len()];
    let mut prev = vec![vec![Option::<Node>::None; graph[0].len()]; graph.len()];

    dist[start.y as usize][start.x as usize] = 0;
    let mut Q = HashSet::<Node>::new();
    for y in 0..graph.len() {
        for x in 0..graph[0].len() {
            Q.insert(Node { x: x as i32, y: y as i32, cell: graph[y][x]});
        }
    }

    while !Q.is_empty() {
        let u = *Q.iter().min_by_key(|node| dist[node.y as usize][node.x as usize]).unwrap();
        assert_ne!(dist[u.y as usize][u.x as usize], i32::MAX);
        if u.cell == Cell::End {
            break;
        }

        Q.remove(&u);

        neighbors(u, graph).filter(|v| Q.contains(v))
            .for_each(|v| {
                let alt = dist[u.y as usize][u.x as usize] + 1;
                if alt <= dist[v.y as usize][v.x as usize] {
                    dist[v.y as usize][v.x as usize] = alt;
                    prev[v.y as usize][v.x as usize] = Some(u);
                }
            });
    }
    (dist, prev)
}

fn dijkstras(graph: &Map, start: Node, target: Node) -> Vec<Node> {
    assert_eq!(start.cell, Cell::Start);
    assert_eq!(target.cell, Cell::End);
    let (dist, prev) = dijkstras0(graph, start);
    let mut S = Vec::new();
    let mut u = target;
    if prev[u.y as usize][u.x as usize].is_some() {
        while let Some(prev_node) = prev[u.y as usize][u.x as usize] {
            S.push(u);
            u = prev_node;
        }
    }
    S
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
    let mut start = Node { x: 0, y: 0, cell: Cell::Start };
    let mut end = Node { x: 0, y: 0, cell: Cell::End };
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let cell = map[y][x];
            if cell == Cell::Start {
                start.x = x as i32;
                start.y = y as i32;
            }
            if cell == Cell::End {
                end.x = x as i32;
                end.y = y as i32;
            }
        }
    }
    let path = dijkstras(&map, start, end);

    println!("path size = {}", path.len());
}
