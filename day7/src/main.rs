use std::collections::HashMap;

#[derive(Debug)]
enum LsEntry<'a> {
    Dir(&'a str),
    File(&'a str, usize)
}

fn parse_ls_line(line: &str) -> LsEntry {
    match line.split_once(' ').unwrap() {
        ("dir", dir) => LsEntry::Dir(dir),
        (size, name) => LsEntry::File(name, size.parse().unwrap())
    }
}

#[derive(PartialEq, Debug)]
enum Cmd<'a> {
    LS,
    CD(&'a str)
}

#[derive(Debug)]
enum NodeData<'a> {
    File(&'a str, usize), // name and size
    Dir(&'a str, Vec<LsEntry<'a>>) // name and names of children
}
#[derive(Debug)]
struct FileNode<'a> {
    parent: Option<&'a str>,
    data: NodeData<'a>
}

type Path<'a> = Vec<&'a str>;
type FileTree<'a> = HashMap<Path<'a>, Box<FileNode<'a>>>;

fn recurse_files<'a, F>(tree: &FileTree<'a>, p: &mut Path<'a>, f: &mut F)
where F: FnMut(&'a str, usize)
{
    let node = &tree.get(p).unwrap().data;

    match node {
        NodeData::File(s, size) => {
            f(s, *size);
        },
        NodeData::Dir(_, children) => {
            for entry in children {
                match entry {
                    LsEntry::Dir(child) => {
                        p.push(child);
                        recurse_files(tree, p, f);
                        p.pop();
                    },
                    LsEntry::File(s, size) => {
                        f(s, *size)
                    }
                };
            }
        }
    }
}

fn recurse_directories<'a, F>(tree: &FileTree<'a>, p: &mut Path<'a>, f: &mut F)
    where F: FnMut(&Path, &'a str, &Vec<LsEntry<'a>>)
{
    let node = &tree.get(p).unwrap().data;

    if let NodeData::Dir(name, children) = node {
        f(p, name, children);

        for entry in children {
            if let LsEntry::Dir(child) = entry {
                p.push(child);
                recurse_directories(tree, p, f);
                p.pop();
            }
        }
    } else {
        panic!()
    }
}

fn compute_dir_size_recursive<'a>(tree: &FileTree<'a>, p: &mut Path<'a>) -> usize {
    let mut total_size = 0usize;
    recurse_files(&tree, p, &mut |_, size| total_size += size);
    return total_size;
}

// TODO: optimize with dynamic programming
fn compute_all_dir_sizes<'a>(tree: &FileTree<'a>) -> Vec<(&'a str, usize)> {
    let mut root = vec![];
    let mut out = vec![];
    recurse_directories(tree, &mut root, &mut |path, name, children| {
        let mut p = path.clone();
        let sum = compute_dir_size_recursive(tree, &mut p);
        out.push((name, sum));
    });
    out
}

fn main() {
    let input = include_str!("../input");

    let mut lines_iter = input.lines();
    assert_eq!(lines_iter.next(), Some("$ cd /"));

    let mut directory_map = FileTree::new();

    let mut cwd = Path::new();
    let mut prev_cmd = Cmd::CD("/");
    let mut current_entries = Vec::<LsEntry>::new();
    loop {
        let next = lines_iter.next();
        // if this is a new command after ls or EOF
        if next.map_or(true, |l| l.starts_with('$') && prev_cmd == Cmd::LS) {
            directory_map.insert(cwd.clone(), Box::new(FileNode {
                parent: cwd.iter().rev().nth(1).map_or(None, |s| Some(*s)),
                data: NodeData::Dir(cwd.last().map_or("/", |s| *s), current_entries),
            }));
            current_entries = Vec::new();
        }
        if next.is_none() { break };

        let line = next.unwrap();
        if line.starts_with('$') {
            let cmd = &line[2..];
            match cmd.split_once(' ') {
                Some(("cd", dir)) => {
                    if dir == ".." {
                        cwd.pop();
                    } else {
                        cwd.push(dir);
                    }
                    prev_cmd = Cmd::CD(dir)
                },
                _ => {
                    assert_eq!(cmd, "ls");
                    prev_cmd = Cmd::LS
                }
            }
        } else {
            // this is ls output
            assert_eq!(prev_cmd, Cmd::LS);
            current_entries.push(parse_ls_line(line));
        }
    }

    let mut root: Path = vec![];
    let total_size = compute_dir_size_recursive(&directory_map, &mut root);
    println!("total size = {}", total_size);

    let sizes = compute_all_dir_sizes(&directory_map);
    let part1 = sizes.iter().copied()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<usize>();
    println!("part 1 = {}", part1);

    const TOTAL_SPACE: usize = 70000000;
    const REQUIRED_FREE_SPACE: usize = 30000000;
    let free_space = TOTAL_SPACE - total_size;
    let to_be_freed = REQUIRED_FREE_SPACE - free_space;
    println!("need to free {} space", to_be_freed);
    let part2 = sizes.iter().copied()
        .filter_map(|(_, size)| if size >= to_be_freed { Some(size) } else { None })
        .min()
        .unwrap();
    println!("part 2 = {}", part2);
}
