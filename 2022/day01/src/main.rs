use std::{fs::File, io::{self, BufReader, BufRead}, path::Path};

fn main() {
    let mut elves = Vec::new();
    elves.push(Vec::new());
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.len() == 0 {
                    elves.push(Vec::new());
                    // println!("<empty line>");
                } else {
                    let last_elf_idx = elves.len() - 1;
                    elves
                        .get_mut(last_elf_idx)
                        .unwrap()
                        .push(l);
                    // println!("{}", l);
                }
            }
        }
        println!("We got {} elves!", elves.len());
        let (most_caloric_elf, calories) = get_most_caloric_elf(elves);
        println!("The most caloric elf is elf number {}, and he has {} calories.", most_caloric_elf, calories);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_elf_calories(elf: &Vec<String>) -> usize {
    let sum = elf
        .into_iter()
        .fold(0, |acc, x| x.parse::<usize>().unwrap_or(0) + acc);
    sum
}

fn get_most_caloric_elf(elves: Vec<Vec<String>>) -> (usize, usize) {
    let mut max_cal: usize = 0;
    let mut elf_idx: usize = 0;
    for (idx, elf) in elves.iter().enumerate() {
        let cal = get_elf_calories(elf);
        if cal > max_cal {
            max_cal = cal;
            elf_idx = idx;
        }
    }
    (elf_idx, max_cal)
}
