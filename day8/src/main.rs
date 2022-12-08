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

fn scenic_score(data: &Vec<Vec<u8>>, tree_x: usize, tree_y: usize) -> i32 {
    let cols = data[0].len();
    let rows = data.len();
    let tree_in = data[tree_y][tree_x];
    let mut score_out = 1;
    if tree_x > 0 {
        let mut score = 0;
        for x in (0..tree_x).rev() {
            let tree = data[tree_y][x];
            score += 1;
            if tree >= tree_in {
                break;
            }
        }
        score_out *= score;
    } else { return 0 }
    if tree_x < cols - 1 {
        let mut score = 0;
        for x in (tree_x + 1)..cols {
            let tree = data[tree_y][x];
            score += 1;
            if tree >= tree_in {
                break;
            }
        }
        score_out *= score;
    } else { return 0 }

    if tree_y > 0 {
        let mut score = 0;
        for y in (0..tree_y).rev() {
            let tree = data[y][tree_x];
            score += 1;
            if tree >= tree_in {
                break;
            }
        }
        score_out *= score;
    } else { return 0 }
    if tree_y < rows - 1 {
        let mut score = 0;
        for y in (tree_y + 1)..rows {
            let tree = data[y][tree_x];
            score += 1;
            if tree >= tree_in {
                break;
            }
        }
        score_out *= score;
    } else { return 0 }

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
