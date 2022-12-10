use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn aoc_generic(input: &String, win_size: usize) -> usize {
    for (i, win) in input.chars().collect::<Vec<_>>().windows(win_size).enumerate() {
        let hset:HashSet<char> = win.iter().cloned().collect();
        if hset.len() == win_size {
            return i + win_size;
        }
    }
    return 0;
}

fn aoc(input: &String) -> usize {
    aoc_generic(&input, 4)
}

fn aoc2(input: &String) -> usize {
    aoc_generic(&input, 14)
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{}", aoc(&contents));
    println!("{}", aoc2(&contents));
}