use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::{HashMap, HashSet}, str::Chars};

fn main() {
    let mut all_priorities = 0;
    if let Ok(lines) = readlines("./input.txt") {
        for chunk in lines
            .into_iter()
                .filter_map(|r| r.ok())
                .collect::<Vec<String>>()
                .chunks(3) {
                   let shared = get_shared(chunk);
                   for s in shared {
                        all_priorities += item_priority(s);
                   }
                }
        println!("Sum of priorities = {}", all_priorities);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_els(s: Chars) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    s.into_iter().for_each(|c| *map.entry(c).or_insert(0) += 1);
    map
}

fn get_intersect(maps: Vec<HashMap<char, i32>>) -> Vec<char> {
    if maps.is_empty() {
        return Vec::new();
    }
    let mut sets = Vec::new();
    for map in maps {
        let mut new_set = HashSet::new();
        map.keys().for_each(|k| { new_set.insert(*k); });
        sets.push(new_set);
    }

    let mut first_set = sets.pop().unwrap();
    
    first_set
        .retain(|x| // <-- retain element
                sets.iter().all(|set| set.contains(x)) // <-- Only if all sets contain that element
        );
    first_set.iter().map(|c| *c).collect()
}

fn get_shared(sacks: &[String]) -> Vec<char> {
    let collected: Vec<HashMap<char, i32>> = sacks
        .into_iter()
        .map(|s| count_els(s.chars()))
        .collect();
    get_intersect(collected)
}

fn item_priority(item: char) -> usize {
    if item.is_uppercase() {
        return usize::from(item as u8) - 38;
    } else {
        return usize::from(item as u8) - 96;
    }
}
