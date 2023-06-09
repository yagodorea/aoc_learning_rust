use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashSet};

fn main() {
    let mut count = 0;
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let pair = parse_pair(l.clone());
                if contains_fully(pair) {
                    // println!("fully contained assignment: {}", l);
                    count += 1;
                }
            }
        }
        println!("{} pairs have fully contained assignments", count);
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

fn contains_fully(sets: Vec<HashSet<usize>>) -> bool {
    // Return true if all sets are contained in the biggest set
    let empty_set = HashSet::<usize>::new();
    let biggest_set = sets
        .iter()
        .fold(
            &empty_set, 
            |acc, x| if x.len() > acc.len() { x } else { acc }
        );
    // Check if union of all sets is equal to the biggest set
    if sets
        .iter()
        .fold(sets[0].clone(),
            |acc, x| x
                        .union(&acc)
                        .cloned()
                        .collect()
        ).eq(&biggest_set) {
        return true;
    }
    false
}
