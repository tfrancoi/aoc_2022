use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;



#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct AocFile<'a> {
    name: &'a str,
    size: usize,
}


fn get_parent(path: &str) -> String {
    let mut splitted:Vec<&str> = path.split("/").collect();
    splitted.pop();
    splitted.pop();
    (splitted.join("/") + "/").to_string()
}

fn get_size(dir: &str, dirs: &HashMap<String, RefCell<HashSet<AocFile>>>) -> usize {
    let mut size:usize = 0;
    for k in dirs.keys() {
        if k.starts_with(dir) {
            for f in dirs[k].borrow().iter() {
                size += f.size;
            }
        }
    }

    size
}

fn aoc(input: &String) -> (usize, usize) {
    let mut dirs:HashMap<String, RefCell<HashSet<AocFile>>> = HashMap::new();
    let mut cur_dir = "".to_string();
    for command in input.split("\n").filter(|&x| x != "") {
        let line:Vec<&str> = command.split(" ").collect();
        
        if line[0] == "$" {
            if line[1] == "cd" {
                if cur_dir == "".to_string() {
                    dirs.insert(line[2].to_string(), RefCell::new(HashSet::new()));
                    cur_dir = line[2].to_string();
                }
                else if line[2] == ".." {
                    cur_dir = get_parent(&cur_dir);
                }
                else {
                    cur_dir = cur_dir  + &line[2].to_string() + "/";
                    let existing_dir = dirs.get(&*cur_dir);
                    if existing_dir == None {
                        dirs.insert(cur_dir.clone(), RefCell::new(HashSet::new()));
                    } 
                }
            }
        }
        else {
            if line[0] != "dir" {
            
                let mut dir = dirs.get(&*cur_dir).unwrap().borrow_mut();
                dir.insert(AocFile {
                    size: line[0].parse().expect("Nan"),
                    name: line[1],
                });
            }
        }
        
    }

    let total:usize = 70000000;
    let cur_size = get_size("/", &dirs);
    let free_need =  cur_size + 30000000 - total;
    let mut sizes:Vec<usize> = Vec::new();
    // let mut final_size = 0;
    for k in dirs.keys() {
        let s = get_size(k, &dirs);
        if s >= free_need {
            sizes.push(s);
        }
    }
    sizes.sort();

    let mut final_size = 0;
    for k in dirs.keys() {
        let s = get_size(k, &dirs);
        if s <= 100000 {
            final_size += s;
        }
    }

    return (final_size, sizes[0]);
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
    // println!("{}", aoc2(&contents));
}