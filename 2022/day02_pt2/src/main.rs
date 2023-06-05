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

fn get_my_play(res: &str, they: &str) -> &'static str {
    match res {
        "X" => match they { "A" => "C", "B" => "A", _ => "B" },
        "Y" => match they { "A" => "A", "B" => "B", _ => "C" },
        "Z" => match they { "A" => "B", "B" => "C", _ => "A" },
        _ => "0",
    }
}

fn game_result(res: &str) -> usize {
    match res {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}

fn calc_points(l: String) -> usize {
    let point_map = HashMap::from([
                                  ("A", 1),
                                  ("B", 2),
                                  ("C", 3),
    ]);
    let split: Vec<&str> = l.split(" ").collect();
    let my_play = get_my_play(split[1], split[0]);
    if my_play.eq("0") {
        panic!("invalid input!");
    }
    let hand_points = point_map.get(my_play).unwrap_or(&0);
    let res = game_result(split[1]);
    let result = res + hand_points;
    result
}
