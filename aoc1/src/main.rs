use std::fs::File;
use std::io::prelude::*;


fn aoc(input: String) -> (i32, i32) {
    let mut total: Vec<i32> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    for line in input.split("\n").map(|x| x.trim()) {
        if line == "" {
            total.push(current.iter().sum());
            current.clear();
        }
        else {
            current.push(line.parse().expect("Parse Error"));
        }
    }
    total.sort();
    total.reverse();
    (total[0], total[0..3].iter().sum())
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(contents));
}
