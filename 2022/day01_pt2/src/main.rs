use std::{fs::File, io::{self, BufReader, BufRead}, path::Path};

fn main() {
    let mut elves: Vec<usize> = Vec::new();
    let mut elf: Vec<usize> = Vec::new();
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.len() == 0 {
                    let cals = elf.iter().fold(0, |acc, x| x + acc);
                    elves.push(cals);
                    elves.sort();
                    elves.reverse();
                    elves.truncate(3);
                    elf.truncate(0);
                } else {
                    let cals = l.parse::<usize>().unwrap_or(0);
                    elf.push(cals);
                }
            }
        }
        let total = elves.iter().fold(0, |acc, x| x + acc);
        println!("The most caloric elves have {:?} calories. Total = {}", elves, total);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

