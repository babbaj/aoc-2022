use std::collections::{HashMap};

fn scan_line<I>(visible: &mut HashMap::<(usize, usize), u8>, mut line: I)
where I: Iterator<Item = ((usize, usize), u8)>
{
    let (first_pos, first_tree) = line.next().unwrap();
    visible.insert(first_pos, first_tree);
    let mut highest = first_tree;
    for (pos, tree) in line {
        if tree > highest {
            highest = tree;
            visible.insert(pos, tree);
        }
    }
}

fn score_for_line<I>(tree_in: u8, mut line: I) -> i32
    where I: Iterator<Item = u8>
{
    let mut out = 0;
    for tree in line {
        out += 1;
        if tree >= tree_in {
            break;
        }
    }
    return out;
}

fn scenic_score(data: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize) -> i32 {
    let cols = data[0].len();
    let rows = data.len();
    let tree_in = data[tree_y][tree_x];
    let mut score_out = 1;
    score_out *= score_for_line(tree_in, ((0..tree_x).rev()).map(|x| data[tree_y][x]));
    score_out *= score_for_line(tree_in, ((tree_x + 1)..cols).map(|x| data[tree_y][x]));

    score_out *= score_for_line(tree_in, ((0..tree_y).rev()).map(|y| data[y][tree_x]));
    score_out *= score_for_line(tree_in, ((tree_y + 1)..rows).map(|y| data[y][tree_x]));

    return score_out;
}

fn main() {
    let input = include_str!("../input");
    // list of rows
    // [y][x]
    let data: Vec<Vec<u8>> = input.lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    let cols = data[0].len();
    let rows = data.len();

    let mut visible_trees = HashMap::<(usize, usize), u8>::new();
    for x in 0..cols {
        let vertical = (0..rows).map(|y| ((x, y), data[y][x]));
        scan_line(&mut visible_trees, vertical.clone());
        scan_line(&mut visible_trees, vertical.rev());
    }
    for y in 0..rows {
        let horizontal = (0..cols).map(|x| ((x, y), data[y][x]));
        scan_line(&mut visible_trees, horizontal.clone());
        scan_line(&mut visible_trees, horizontal.rev());
    }
    println!("visible len = {}", visible_trees.len());

    let highest_score = visible_trees.iter()
        .map(|((x, y), _)| ((x, y), scenic_score(&data, *x, *y)))
        .max_by_key(|(_, score)| *score)
        .unwrap();
    println!("highest score = {:?}", highest_score);
}
