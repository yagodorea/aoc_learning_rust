use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashSet, time::Instant};
use colored::Colorize;

#[derive(Debug)]
struct TreeMap {
    grid: Vec<Vec<usize>>,
}

type Tree = (usize, usize);

const DEBUG: bool = false;

fn main() {
    let now = Instant::now();
    let mut buf = Vec::new();
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                buf.push(l);
            }
        }
        let tree_map = create_tree_map(buf);
        if DEBUG { print_tree(&tree_map, &HashSet::new()) };
        let visible_trees = get_visible_trees(&tree_map);
        if DEBUG {
            println!("{} visible_trees -> {:?}", visible_trees.len().to_string().bold(), &visible_trees);
            print_tree(&tree_map, &visible_trees);
        } else {
            println!("{} visible_trees", visible_trees.len().to_string().bold());
        }
    } else {
        println!("Could not read file!");
    }
    let elapsed = now.elapsed();
    println!("Finished in {:.2?}", elapsed);
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_visible_trees(tree_map: &TreeMap) -> HashSet<Tree> {
    let mut visible_trees = HashSet::new();

    for (i, row) in tree_map.grid.iter().enumerate() {
        if i == 0 || i == tree_map.grid.len() - 1 {
            // Add whole row
            let all: Vec<Tree> = row.iter().enumerate().map(|(j,_)| (i,j)).collect();
            visible_trees.extend(all);
            continue;
        }
        visible_from_left(i, row, &mut visible_trees, false);
        visible_from_right(i, row, &mut visible_trees, false);
    }
    let mut transposed_tree = TreeMap { grid: vec![Vec::new(); tree_map.grid.get(0).unwrap().len()] };
    
    for row in &tree_map.grid {
        // For every row of the tree, push an element to the colunn of the transposed tree
        for (i, col) in row.iter().enumerate() {
            transposed_tree.grid.get_mut(i).unwrap().push(*col);
        }
    }
    let mut transposed_visible_trees: HashSet<Tree> = visible_trees.iter().map(|(a,b)| (*b,*a)).collect();

    for (i, row) in transposed_tree.grid.iter().enumerate() {
        if i == 0 || i == tree_map.grid.len() - 1 {
            // Add whole row
            let all: Vec<Tree> = row.iter().enumerate().map(|(j,_)| (i,j)).collect();
            transposed_visible_trees.extend(all);
            continue;
        }
        visible_from_left(i, row, &mut transposed_visible_trees, true);
        visible_from_right(i, row, &mut transposed_visible_trees, true);
    }
    transposed_visible_trees.iter().map(|(a,b)| (*b,*a)).collect()
}

fn visible_from_left(i: usize, tree_row: &Vec<usize>, visible: &mut HashSet<Tree>, inverted: bool) {
    let mut it = tree_row.iter();
    let first = it.next().unwrap();
    visible.insert((i, 0));
    let mut max = *first;
    let dir = match inverted { true => "top", false => "left" };
    for (x, el) in it.enumerate() {
        let j = x + 1;
        if *el > max {
            max = *el;
            let ins = visible.insert((i, j));
            if DEBUG {
                println!("{:?} is visible from the {}", (i, j), dir);
                if !ins { println!("Tree already marked!"); }
            }
        }
    }
}

fn visible_from_right(i: usize, tree_row: &Vec<usize>, visible: &mut HashSet<Tree>, inverted: bool) {
    let mut reversed = tree_row.clone();
    reversed.reverse();
    let len = tree_row.len();
    let mut it = reversed.iter();
    let first = it.next().unwrap();
    visible.insert((i, len - 1));
    let mut max = *first;
    let dir = match inverted { true => "down", false => "bottom" };
    for (x, el) in it.enumerate() {
        // Reversal traversing minus first element
        let j = (len - 1) - (x + 1);
        if *el > max {
            max = *el;
            let ins = visible.insert((i, j));
            if DEBUG {
                println!("{:?} is visible from the {}", (i, j), dir);
                if !ins { println!("Tree already marked!"); }
            }
        }
    }
}

fn print_tree(tree_map: &TreeMap, visible_trees: &HashSet<Tree>) {
    println!("TreeMap:");
    for (i, row) in tree_map.grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let tree = format!("{} ", &col);
            if visible_trees.contains(&(i, j)) {
                print!("{}", tree.bold().on_red().clone());
            } else {
                print!("{}", tree);
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn create_tree_map(raw: Vec<String>) -> TreeMap {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for s in raw {
        let row: Vec<usize> = s.chars().into_iter().map(|n| n.to_string().parse::<usize>().unwrap()).collect();
        grid.push(row);
    }
    return TreeMap { grid };
}


