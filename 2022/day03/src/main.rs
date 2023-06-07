use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::{HashMap, HashSet}};

fn main() {
    let mut all_priorities = 0;
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let els = get_shared(l);
                let priorities = els.into_iter().fold(0, |acc, el| acc + item_priority(el));
                all_priorities += priorities;
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

fn count_els(s: &[char]) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    s.into_iter().for_each(|c| *map.entry(*c).or_insert(0) += 1);
    map
}

fn get_intersect(a: HashMap<char, i32>, b: HashMap<char, i32>) -> Vec<char> {
    let mut set_a = HashSet::new();
    let mut set_b = HashSet::new();
    a.keys().for_each(|k| { set_a.insert(*k); });
    b.keys().for_each(|k| { set_b.insert(*k); });
    let intersection = set_a.intersection(&set_b);
    intersection.copied().collect()
}

fn get_shared(sack: String) -> Vec<char> {
    let chars = sack.chars().collect::<Vec<char>>();
    let pivot = chars.len() / 2;
    let (first_comp, second_comp) = chars.split_at(pivot);
    let fels = count_els(first_comp);
    let sels = count_els(second_comp);
    get_intersect(fels, sels)
}

fn item_priority(item: char) -> usize {
    if item.is_uppercase() {
        return usize::from(item as u8) - 38;
    } else {
        return usize::from(item as u8) - 96;
    }
}
