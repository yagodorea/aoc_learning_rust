use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashSet};

fn main() {
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                println!("datastream buffer: {}", l);
                println!("marker at {}", get_marker(l));
            }
        }
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_unique(ch: &Vec<char>) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(ch.iter());
    return set.len() == ch.len();
}

fn get_marker(buffer: String) -> usize {
    let mut chrs = Vec::new();
    let mut i: usize = 0;
    for c in buffer.chars() {
        i += 1;
        chrs.push(c);
        if i >= 14 {
            if is_unique(&chrs) {
                return i;
            } else {
                chrs.remove(0);
            }
        }
    }
    i
}
