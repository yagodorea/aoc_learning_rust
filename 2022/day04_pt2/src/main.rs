use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashSet};

fn main() {
    let mut count = 0;
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let pair = parse_pair(l.clone());
                if has_intersection(pair) {
                    println!("has intersection: {}", l);
                    count += 1;
                }
            }
        }
        println!("{} pairs have intersecting assignments", count);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_pair(s: String) -> Vec<HashSet<usize>>  {
    let mut pair = Vec::new();
    let elves: Vec<&str> = s.split(',').into_iter().collect();
    for elf in elves {
        let bounds: Vec<usize> = elf
            .split('-')
            .map(|v| v.parse().unwrap())
            .collect();
        let (a, b) = (bounds[0], bounds[1]);
        let mut elf_block = HashSet::new();
        for x in a..b+1 {
            elf_block.insert(x);
        }
        pair.push(elf_block);
    }
    pair
}

fn has_intersection(sets: Vec<HashSet<usize>>) -> bool {
    // Return true if there's an intersection between all the sets
    let mut _sets = sets.clone();
    let mut first_set = _sets.pop().unwrap();
    first_set.retain(|el| 
                     _sets
                     .iter()
                     .all(|set| set.contains(el)));
    return first_set.len() > 0;
}
