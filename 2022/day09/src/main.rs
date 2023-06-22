use std::{fs::File,io::{self,BufReader,BufRead},path::Path, collections::HashSet, ops::Deref, time::Instant};

type Pos = (i32, i32);

const DEBUG: bool = false;

fn main() {
    let now = Instant::now();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                process_move(l, &mut head, &mut tail, &mut visited);
            }
        }
        println!("Positions visited by tail: {}.", visited.len());
    } else {
        println!("Could not read file!");
    }
    let elapsed = now.elapsed();
    println!("Finished in {:.2?}", elapsed);
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_vicinity(pos: &Pos) -> [Pos; 9] {
    let mut vicinity = [(0, 0); 9];
    for i in 0..3 {
        for j in 0..3 {
            let idx: usize = (i * 3) + j;
            let npos = (pos.0-1 + (i as i32), pos.1-1 + (j as i32));
            vicinity[idx] = npos;
        }
    }
    vicinity
}

fn unit(n: i32) -> i32 {
    if n > 0 { return 1; }
    -1
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn move_once(origin: &Pos, target: &Pos) -> Pos {
    let xd = target.0 - origin.0;
    let yd = target.1 - origin.1;
    if xd.abs() > 0 && yd.abs() > 0 {
        let to = add(*origin, (unit(xd), unit(yd)));
        return to;
    } else {
        let toy = (0, unit(yd));
        let tox = (unit(xd), 0);
        return match xd {
            0 => { add(*origin, toy) },
            _ => { add(*origin, tox) },
        };
    }
}

fn adjust_tail(head: &Pos, tail: &Pos) -> Pos {
    // Check if tail is in the vicinity of the head
    // If it's not, then move the tail straightly or diagonally
    let vic = get_vicinity(head);
    if vic.contains(tail) {
        return *tail;
    }
    move_once(tail, head)
}

fn head_next_pos(head: &Pos, dir: &str) -> Pos {
    match dir {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "L" => (head.0 - 1, head.1),
        "R" => (head.0 + 1, head.1),
        _ => (0, 0), // Shouldn't be reachable
    }
}

fn print_state(h: &Pos, t: &Pos, visited: &HashSet<Pos>) {
    let mut final_str: Vec<String> = Vec::new();
    let x0 = visited.iter().map(|p| p.0).min().unwrap_or(0).min(h.0).min(t.0);
    let x1 = visited.iter().map(|p| p.0).max().unwrap_or(0).max(h.0).max(t.0) + 1;
    let y0 = visited.iter().map(|p| p.1).min().unwrap_or(0).min(h.1).min(t.1);
    let y1 = visited.iter().map(|p| p.1).max().unwrap_or(0).max(h.1).max(t.1) + 1;
    let mut row = String::from("");
    row.push_str(&format!("└"));
    for _ in 0..x1-x0 { row.push_str(&format!("-")); }
    row.push_str(&format!("┘\n"));
    for y in y0..y1 {
        final_str.push(row.clone());
        row.clear();
        row.push_str(&format!("|"));
        for x in x0..x1 {
            if *h == (x, y) { row.push_str(&format!("H")); }
            else if *t == (x, y) { row.push_str(&format!("T")); }
            else if visited.contains(&(x, y)) { row.push_str(&format!("#")); }
            else { row.push_str(&format!(".")); }
        }
        row.push_str(&format!("|\n"));
    }
    final_str.push(row.clone());
    row.clear();
    row.push_str(&format!("┌"));
    for _ in 0..x1-x0 { row.push_str(&format!("-")); }
    row.push_str(&format!("┐\n"));
    final_str.push(row);
    final_str.reverse();
    print!("{}", final_str.join(""));
}

fn process_move(mv: String, head: &mut Pos, tail: &mut Pos, visited: &mut HashSet<Pos>) {
    let parts: Vec<&str> = mv.split(" ").into_iter().collect();
    let dir = parts.get(0).unwrap().deref();
    let num = parts.get(1).unwrap().deref();
    if DEBUG {
        println!("Moving head {} positions in direction {}", num, dir);
        print_state(head, tail, visited);
    }
    for _ in 0..num.parse().unwrap() {
        *head = head_next_pos(head, dir);
        *tail = adjust_tail(&head, &tail);
        visited.insert(tail.clone());
    }
    if DEBUG {
        print_state(head, tail, visited);
    }
}
