use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashMap, time::Instant};
use colored::Colorize;

#[derive(Debug)]
struct TreeMap {
    grid: Vec<Vec<usize>>,
}

#[derive(Clone, Copy, Debug)]
struct TreeView {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}
type TreePosition = (usize, usize);
#[derive(Debug)]
struct Tree {
    position: TreePosition,
    view: TreeView,
    is_visible: bool,
}

const DEBUG: bool = false;

fn main() {
    let now = Instant::now();
    let mut buf = Vec::new();
    let mut trees: HashMap<TreePosition, Tree> = HashMap::new();
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                buf.push(l);
            }
        }
        let tree_map = create_tree_map(buf);
        if DEBUG {
            println!("TreeMap:");
            print_tree(&tree_map, &trees)
        };
        process_tree_map(&tree_map, &mut trees);
        let visible_trees: Vec<TreePosition> = trees.iter().filter(|t| t.1.is_visible).map(|(pos,_)| *pos).collect();
        if DEBUG {
            println!("{} visible_trees -> {:?}", visible_trees.len().to_string().bold(), &visible_trees);
            println!("TreeMap:");
            print_tree(&tree_map, &trees);
        } else {
            println!("{} visible_trees", visible_trees.len().to_string().bold());
        }
        print_vision_map(&trees);
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

fn process_tree_map(tree_map: &TreeMap, trees: &mut HashMap<TreePosition, Tree>) {
    let zeroview = TreeView { left: 0, right: 0, top: 0, bottom: 0 };
    for (i, row) in tree_map.grid.iter().enumerate() {
        if i == 0 || i == tree_map.grid.len() - 1 {
            // Add whole row
            let all: Vec<(TreePosition, Tree)> = row
                .iter()
                .enumerate()
                .map(|(j,_)| (
                        (i,j),
                        Tree { position: (i, j), is_visible: true, view: zeroview }
                        )).collect();
            trees.extend(all);
            continue;
        }
        visible_from_left(i, row, trees, false);
        visible_from_right(i, row, trees, false);
    }
    let mut transposed_tree = TreeMap { grid: vec![Vec::new(); tree_map.grid.get(0).unwrap().len()] };
    
    for row in &tree_map.grid {
        // For every row of the tree, push an element to the colunn of the transposed tree
        for (i, col) in row.iter().enumerate() {
            transposed_tree.grid.get_mut(i).unwrap().push(*col);
        }
    }

    for (i, row) in transposed_tree.grid.iter().enumerate() {
        if i == 0 || i == tree_map.grid.len() - 1 {
            // Add whole row
            let all: Vec<(TreePosition, Tree)> = row
                .iter()
                .enumerate()
                .map(|(j,_)| (
                        (j,i),
                        Tree { position: (j, i), is_visible: true, view: zeroview }
                        )).collect();
            trees.extend(all);
            continue;
        }
        visible_from_left(i, row, trees, true);
        visible_from_right(i, row, trees, true);
    }
}

fn get_vision(tree_h: usize, trail: &Vec<usize>) -> usize {
    let mut vision = 1;
    let mut reversed_trail = trail.clone();
    reversed_trail.reverse();
    reversed_trail.pop();
    for tree in reversed_trail {
        if tree >= tree_h {
            return vision;
        }
        vision += 1;
    }
    vision
}

fn visible_from_left(i: usize, tree_row: &Vec<usize>, trees: &mut HashMap<TreePosition, Tree>, inverted: bool) {
    let mut it = tree_row.iter();
    let first = it.next().unwrap();
    //let dir = match inverted { true => "top", false => "left" };
    //println!("Passing from the {}, line/col {}", dir, i);
    let mut view = TreeView {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    if inverted {
        trees.entry((0, i)).and_modify(|t| t.view.top = 0);
    } else {
        trees.entry((i, 0)).and_modify(|t| t.view.left = 0)
            .or_insert(Tree { position: (i, 0), is_visible: true, view });
    }
    let mut trail = vec![*first];
    for (x, el) in it.enumerate() {
        let j = x + 1;
        let vision = get_vision(*el, &trail);
        //println!("tree {}, vision {} from the {}, trail -> {:?}", el, vision, dir, trail);
        if inverted { view.top = vision; } else { view.left = vision; };
        let max = *trail.iter().max().unwrap();
        trail.push(*el);
        let mut is_visible = true;
        if *el <= max {
            view.left = vision;
            is_visible = false;
        }
        if inverted {
            //println!("Setting vision {} to pos {:?} (v={})", vision, ((j, i)), el);
            trees.entry((j, i)).and_modify(|t| {
                if is_visible { t.is_visible = true; }
                t.view.top = vision;
            });
        } else {
            //println!("Setting vision {} to pos {:?} (v={})", vision, ((i, j)), el);
            trees.entry((i, j)).and_modify(|t| {
                if is_visible { t.is_visible = true; }
                t.view.left = vision;
            }).or_insert(Tree { position: (i, j), is_visible, view });
        }
    }
}

fn visible_from_right(i: usize, tree_row: &Vec<usize>, trees: &mut HashMap<TreePosition, Tree>, inverted: bool) {
    let mut reversed = tree_row.clone();
    reversed.reverse();
    //let dir = match inverted { true => "bottom", false => "right" };
    let len = tree_row.len();
    //println!("Passing from the {}, line/col {}", dir, i);
    let mut it = reversed.iter();
    let first = it.next().unwrap();
    let mut view = TreeView {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    };
    if inverted {
        trees.entry((len-1, i)).and_modify(|t| t.view.bottom = 0);
    } else {
        trees.entry((i, len-1)).and_modify(|t| t.view.right = 0)
            .or_insert(Tree { position: (i, len-1), is_visible: true, view });
    }
    let mut trail = vec![*first];
    for (x, el) in it.enumerate() {
        // Reversal traversing minus first element
        let j = (len - 1) - (x + 1);
        let vision = get_vision(*el, &trail);
        //println!("tree {}, vision {} from the {}, trail -> {:?}", el, vision, dir, trail);
        if inverted { view.bottom = vision; } else { view.right = vision; };
        let max = *trail.iter().max().unwrap();
        trail.push(*el);
        let mut is_visible = true;
        if *el <= max {
            view.right = vision;
            is_visible = false;
        }
        if inverted {
            //println!("Setting vision {} to pos {:?} (v={})", vision, ((j, i)), el);
            trees.entry((j, i)).and_modify(|t| {
                if is_visible { t.is_visible = true; }
                t.view.bottom = vision;
            });
        } else {
            //println!("Setting vision {} to pos {:?} (v={})", vision, ((i, j)), el);
            trees.entry((i, j)).and_modify(|t| {
                if is_visible { t.is_visible = true; }
                t.view.right = vision;
            }).or_insert(Tree { position: (i, j), is_visible, view });
        }
    }
}

fn print_tree(tree_map: &TreeMap, trees: &HashMap<TreePosition, Tree>) {
    for (i, row) in tree_map.grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let tree = format!("{} ", &col);
            if trees.contains_key(&(i,j)) && trees.get(&(i,j)).unwrap().is_visible {
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

fn vision_product(view: &TreeView) -> usize {
    view.left * view.right * view.top * view.bottom
}

fn print_vision_map(trees: &HashMap<TreePosition, Tree>) {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    let dimensions = trees.keys().into_iter().max_by_key(|k| k.0 + k.1).unwrap();
    grid.extend(vec![Vec::new(); dimensions.0 + 1]);
    for (i,row) in grid.iter_mut().enumerate() {
        let mut cols: Vec<usize> = Vec::new();
        for j in 0..dimensions.1 + 1 {
            cols.push(vision_product(&(trees.get(&(i, j)).unwrap().view)));
        }
        row.extend(cols);
    }

    let mapped_trees: HashMap<TreePosition, Tree> = trees.iter().map(
        |(k,t)| 
        (*k, Tree { is_visible: false, view: t.view, position: t.position })
    ).collect();

    if DEBUG {
        println!("VisionMap:");
        print_tree(&(TreeMap { grid: grid.clone().iter().map(|r| r.iter().map(|el| *el).collect::<Vec<usize>>()).collect() }), &mapped_trees);
    }

    println!("Greatest vision:");
    let greatest_vision = grid.iter().flatten().max().unwrap_or(&0);
    println!("{}", greatest_vision.to_string().bold());
}
