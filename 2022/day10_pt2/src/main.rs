use std::{fs::File,io::{self,BufReader,BufRead},path::Path};

fn main() {
    let mut cmds = Vec::new();
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                cmds.push(l);
            }
        }
        let (signal_strength, screen) = process_commands(cmds);
        println!("Signal strength = {}", signal_strength);
        print_screen(&screen);
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

enum Command {
    Addx(i32),
    Noop,
}

fn parse_cmd(cmd: String) -> Command {
    let parts: Vec<String> = cmd.split(" ").map(|s| String::from(s)).collect();
    if parts.get(0).unwrap() == "noop" {
        return Command::Noop;
    } else {
        let value: i32 = parts.get(1).unwrap().parse().unwrap();
        return Command::Addx(value);
    }
}

fn print_register_history(x: Vec<i32>) {
    let max = x.iter().max().unwrap_or(&0);
    let min = x.iter().min().unwrap_or(&0);

    println!("max: {}, min: {}", max, min);
    let mut inverted_str: Vec<String> = Vec::new();
    for i in *min..*max+1 {
        // Print row
        let mut line = String::from("");
        for j in 0..x.len() {
            let val = x.get(j).unwrap().clone();
            if i == 0 {
                if val == i {
                    line.push_str("▤");
                } else {
                    line.push_str(".");
                }
                continue;
            }
            if val >= i && i > 0 {
                line.push_str("■");
                continue;
            } else if val <= i && i < 0 {
                line.push_str("□");
                continue;
            } else {
                line.push_str(" ");
            }
        }
        inverted_str.push(line);
    }
    inverted_str.reverse();
    println!("{}", inverted_str.join("\n"));
}

fn draw_pixel(x: i32, cycle: i32) -> char {
    let c = (cycle - 1) % 40;
    if c >= x-1 && c <= x+1 {
        return '#';
    }
    '.'
}

fn print_screen(screen: &Vec<String>) {
    for row in screen {
        println!("{}", row);
    }
}

fn process_commands(cmds: Vec<String>) -> (i32, Vec<String>) {
    let mut X = vec![1];
    let mut signal_strength = 0;
    let mut signal_sum = 0;
    let mut cycle = 0;
    let mut next_eval = 1;
    let mut add_to_x = 0;
    let mut _cmds = cmds.clone();
    let mut screen: Vec<String> = Vec::new();
    let mut row = String::from("");
    _cmds.reverse();
    while _cmds.len() > 0 || next_eval > cycle + 1 {
        cycle += 1;
        let mut xhead = X.get(X.len() - 1).unwrap().clone();

        let end_of_operation = next_eval - cycle == 1;
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            let mut increase = xhead;
            // Make up for this happening here instead of after the operation finishes
            if !end_of_operation { increase += add_to_x; }
            //print!("Cycle {} ->> Signal strength increased from {} to {}*{} = ", cycle, signal_strength, cycle, increase);
            signal_strength = cycle * increase;
            signal_sum += signal_strength;
            //println!("{}", signal_strength);
        }
        if cycle > 1 && (cycle-1) % 40 == 0 {
            screen.push(row.clone());
            //println!("=====> finish row");
            row.clear();
        }
        if next_eval > cycle {
            X.push(xhead);
            //println!("cycle {} [X={}]", cycle, xhead);
            row.push_str(&draw_pixel(xhead, cycle).to_string());
            //println!("-----------");
            continue;
        }
        next_eval = cycle + 1;
        xhead += add_to_x;
        X.push(xhead);
        add_to_x = 0;
        //print!("cycle {} eval", cycle);
        let cmd = _cmds.pop().unwrap();
        match parse_cmd(cmd) {
            Command::Noop => {
                //println!(" > noop [X={}]", xhead);
                next_eval = cycle + 1;
            },
            Command::Addx(v) => {
                //println!(" > addx {} [X={}]", v, xhead);
                add_to_x = v;
                next_eval = cycle + 2;
            }
        }
        row.push_str(&draw_pixel(xhead, cycle).to_string());
        //println!("-----------");
    }
    // Last cycle
    let mut xhead = X.get(X.len() - 1).unwrap().clone();
    xhead += add_to_x;
    X.push(xhead);
    //println!("after cycle {} [X={}]", cycle, xhead);
    screen.push(row);
    //print_register_history(X);
    // print_screen(&screen);
    (signal_sum, screen)
}
