use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashMap};

fn main() {
    let mut heaps: HashMap<usize, Vec<String>> = HashMap::new();
    let mut initial_state = Vec::new();
    let mut processed_initial_state = false;
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if l.len() == 0 {
                    heaps = process_initial_state(initial_state.clone());
                    processed_initial_state = true;
                } else {
                    if !processed_initial_state {
                        initial_state.push(l);
                        continue;
                    }
                    crane_move(&mut heaps, l);
                }
            }
        }
        println!("final heap state:");
        print_heaps(&heaps);
        println!("top crates = {}", get_top_crates(&heaps));
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_initial_state(mut state: Vec<String>) -> HashMap<usize, Vec<String>> {
    println!("initial state:\n{}", state.clone().join("\n"));
    let mut heaps = HashMap::new();
    let stacks: Vec<usize> = state
        .pop()
        .unwrap()
        .trim()
        .split(' ')
        .filter(|c| !c.is_empty())
        .map(|c| c.parse::<usize>().unwrap())
        .collect();
    while state.len() > 0 {
        let crate_line = state.pop().unwrap();
        for s in stacks.clone() {
            let idx = (s - 1) * 4;
            let cr = crate_line[idx..idx + 3].trim();
            if !cr.is_empty() {
                heaps.entry(s).or_insert(Vec::<String>::new()).push(String::from(cr));
            }
        }
    }
    heaps
}

fn crane_move(heaps: &mut HashMap<usize, Vec<String>>, cmd: String) -> () {
    println!("{}", cmd);
    let cmd_parts: Vec<&str> = cmd.split(" ").into_iter().collect();
    let moves: usize = cmd_parts.get(1).unwrap().parse().unwrap();
    let from: usize = cmd_parts.get(3).unwrap().parse().unwrap();
    let to: usize = cmd_parts.get(5).unwrap().parse().unwrap();
    let mut buf = Vec::new();
    for _ in 0..moves {
        let el = heaps.get_mut(&from).unwrap().pop();
        if let Some(s) = el {
            buf.push(s);
        }
    }
    buf.reverse();
    heaps.get_mut(&to).unwrap().append(&mut buf);
    println!("state:");
    print_heaps(heaps);
}

fn print_heaps(heaps: &HashMap<usize, Vec<String>>) -> () {
    let mut full_print = String::from("");
    let cols = heaps.keys().len();
    let max_height = heaps.values().into_iter().map(|h| h.len()).max().unwrap();
    for i in 0..max_height {
        let idx = max_height - i - 1;
        for heap_idx in 1..cols+1 {
            let heap = heaps.get(&heap_idx).unwrap();
            let cargo = heap.get(idx);
            if let Some(c) = cargo {
                //println!("Element {} of heap {} is {}", idx, heap_idx, c);
                full_print.push_str(format!("{} ", c).as_str());
            } else {
                //println!("Did not find element {} on heap {}", idx, heap_idx);
                full_print.push_str("    ");
            }
        }
        full_print.push_str("\n");
    }
    for i in 0..cols {
        full_print.push_str(format!(" {}  ", i+1).as_str());
    }
    println!("{}", full_print);
}

fn get_top_crates(heaps: &HashMap<usize, Vec<String>>) -> String {
    let mut top_crates = String::from("");
    for idx in 0..heaps.len() {
        let heap = heaps.get(&(idx + 1)).unwrap();
        if heap.is_empty() { continue; }
        let entry = heap.get(heap.len() - 1);
        if let Some(s) = entry {
            top_crates.push_str(s);
        }
    }
    top_crates.replace("[", "").replace("]", "")
}
