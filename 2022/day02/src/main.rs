use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashMap};

fn main() {
    let mut points: usize = 0;
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.len() != 0 {
                    points += calc_points(l);
                }
            }
        }
        println!("Total points: {}", points);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn game_result(me: &str, they: &str) -> usize {
    match me {
        "X" => match they { "B" => 0, "C" => 6, _ => 3 },
        "Y" => match they { "C" => 0, "A" => 6, _ => 3 },
        "Z" => match they { "A" => 0, "B" => 6, _ => 3 },
        _ => 3,
    }
}

fn calc_points(l: String) -> usize {
    let point_map = HashMap::from([
                                  ("X", 1),
                                  ("Y", 2),
                                  ("Z", 3),
    ]);
    let split: Vec<&str> = l.split(" ").collect();
    let hand_points = point_map.get(split[1]).unwrap_or(&0);
    if hand_points == &0 {
        println!("handpoints = 0!!");
    }
    let result = game_result(split[1], split[0]) + hand_points;
    result
}
