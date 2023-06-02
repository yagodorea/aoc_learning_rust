use std::{fs::File,io::{self,BufReader,BufRead},path::Path};

fn main() {
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.len() == 0 {
                    println!("<empty line>");
                } else {
                    println!("{}", l);
                }
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
