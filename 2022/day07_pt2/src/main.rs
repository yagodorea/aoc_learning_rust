use std::{fs::File as FsFile,io::{self,BufReader,BufRead},path::Path, collections::HashMap};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Folder {
    name: String,
    folders: HashMap<String, Folder>,
    files: HashMap<String, File>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    current_dir: String,
    folders: HashMap<String, Folder>,
    files: HashMap<String, File>,
}

fn add_folder(folders: &mut HashMap<String, Folder>, name: String) {
    folders.insert(name.clone(), Folder { folders: HashMap::new(), files: HashMap::new(), name });
}

fn add_file(files: &mut HashMap<String, File>, name: String, size: usize) {
    files.insert(name.clone(), File { size, name });
}

const max_used_space: usize = 40_000_000;

fn main() {
    let mut res_buf = Vec::new();
    let mut device = Device{ current_dir: String::from("/"), folders: HashMap::new(), files: HashMap::new() };
    if let Ok(lines) = readlines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let first_char = l.chars().into_iter().next().unwrap();
                if first_char == '$' {
                    if res_buf.len() > 0 {
                        eval_ls(&res_buf, &mut device);
                        res_buf.clear();
                    }
                    eval_cmd(l, &mut device);
                } else {
                    res_buf.push(l);
                }
            }
        }
        if res_buf.len() > 0 {
            eval_ls(&res_buf, &mut device);
            res_buf.clear();
        }
        // println!("{:#?}", device);
        let all_folders = list_all_folders(&device);
        let mut big_folders: HashMap<String, usize> = HashMap::new();
        for f in all_folders.clone() {
            let size = calc_dir_size(f);
            if size < 100_000 {
                big_folders.insert(f.name.clone(), size);
            }
        }
        println!("Folders smaller than 100kb: {:#?}", big_folders);
        println!("Sum -> {}", big_folders.values().sum::<usize>());
        let total_size = get_device_total_size(&device);
        println!("Total device size: {}", total_size);
        let space_to_be_freed = total_size - max_used_space;
        if space_to_be_freed > 0 {
            println!("Need to free {}", space_to_be_freed);
            let mut sorted_folders: Vec<(String, usize)> = all_folders.iter().map(|f| (f.name.clone(), calc_dir_size(f))).collect();
            sorted_folders.sort_by_key(|f| f.1);
            for sf in sorted_folders {
                if sf.1 > space_to_be_freed {
                    println!("Need to free directory {} of size {}.", sf.0, sf.1);
                    return;
                }
            }
        }
    } else {
        println!("Could not read file!");
    }
}

fn readlines<P>(fname: P) -> io::Result<io::Lines<BufReader<FsFile>>> where P: AsRef<Path> {
    let file = FsFile::open(fname)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_device_total_size(device: &Device) -> usize {
    let folders_size: usize = device.folders.values().map(|f| calc_dir_size(f)).sum();
    let files_size: usize = device.files.values().map(|f| f.size).sum();
    folders_size + files_size
}

fn list_all_folders(device: &Device) -> Vec<&Folder> {
    let mut folders = Vec::new();
    for f in device.folders.values() {
        folders.push(f);
        let sub_folders = get_sub_folders(f);
        folders.extend(sub_folders);
    }
    folders
}

fn get_sub_folders(f: &Folder) -> Vec<&Folder> {
    let mut folders = Vec::new();
    for subf in f.folders.values() {
        folders.push(subf);
        let subsub = get_sub_folders(subf);
        folders.extend(subsub);
    }
    folders
}

fn calc_dir_size(folder: &Folder) -> usize {
    let mut total_size = 0;
    for file in folder.files.values() {
       total_size += file.size; 
    }
    for subf in folder.folders.values() {
        total_size += calc_dir_size(subf);
    }
    total_size
}

fn eval_cd(dir: String, device: &mut Device) {
    match dir.as_str() {
        "/" => { device.current_dir = dir; }
        ".." => {
            let mut paths: Vec<String> = device.current_dir
                .split("/")
                .filter(|s| !s.is_empty())
                .map(|s| String::from(s))
                .collect();
            paths.pop();
            if paths.is_empty() { device.current_dir = String::from("/"); }
            else {
                let mut joined = paths.join("/");
                joined = format!("/{}/", joined);
                device.current_dir = joined.clone();
            }

        },
        d => {
            let mut _d = String::from(d);
            _d.push_str("/");
            device.current_dir.push_str(_d.as_str());
        }
    }
}

fn eval_ls(res: &Vec<String>, device: &mut Device) {
    let mut folders: Vec<String> = device.current_dir.split("/").filter(|p| !p.is_empty()).map(|s| String::from(s)).collect();
    if folders.len() == 0 {
        // add files and folders to folder root
        for obj in res {
            let mut parts: Vec<&str> = obj.split(" ").collect();
            let fp = parts.get(0).unwrap().clone();
            if fp == "dir" {
                // folder
                let name = parts.pop().unwrap().to_string();
                let mut fqp: Vec<String> = device.current_dir.split("/").filter(|p| !p.is_empty()).map(|s| String::from(s)).collect();
                fqp.push(name);
                let fqn = fqp.join(".");
                add_folder(&mut device.folders, fqn);
            } else {
                // file
                let name = parts.pop().unwrap().to_string();
                let size: usize = parts.pop().unwrap().parse().unwrap();
                add_file(&mut device.files, name, size);
            }
        }
        return;
    }
    let first_folder = folders.get(0).unwrap().clone();
    folders.remove(0);
    let mut folder_ref: &mut Folder = device.folders.get_mut(&first_folder).unwrap();
    let mut path = String::from(first_folder);
    for folder in folders {
        path = vec![path, folder].join(".");
        folder_ref = folder_ref.folders.get_mut(&path).unwrap();
    }
    // add files and folders to folder
    for obj in res {
        let mut parts: Vec<&str> = obj.split(" ").collect();
        let fp = parts.get(0).unwrap().clone();
        if fp == "dir" {
            // folder
            let name = parts.pop().unwrap().to_string();
            let mut fqp: Vec<String> = folder_ref.name.split(".").map(|s| String::from(s)).collect();
            fqp.push(name);
            let fqn = fqp.join(".");
            add_folder(&mut folder_ref.folders, fqn);
        } else {
            // file
            let name = parts.pop().unwrap().to_string();
            let size: usize = parts.pop().unwrap().parse().unwrap();
            add_file(&mut folder_ref.files, name, size);
        }
    }
}

fn eval_cmd(cmd: String, device: &mut Device) {
    let mut parts: Vec<String> = cmd.split_whitespace().into_iter().map(|s| String::from(s)).collect();
    let first = parts.get(1).unwrap();
    match first.as_str() {
        "cd" => {
            let dir = parts.pop().unwrap();
            eval_cd(dir, device);
        },
        "ls" => {},
        c => { println!("Invalid command {c}"); }
    }
}

fn print_device(device: &Device) -> String {
    let jsn = serde_json::to_string(device).unwrap();
    format!("{}", jsn).to_owned()
}
