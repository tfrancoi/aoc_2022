use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn get_priority(c: char) -> u32 {
    let char_value: u32 = c.into();
    if c.is_uppercase() {
        return char_value - 38;
    }
    else {
        return char_value - 96;
    }
}

fn aoc(input: &String) -> u32 {
    let mut score: u32 = 0;
    for line in input.split("\n").filter(|x| *x != "") {
        let rucksacks: Vec<char> = line.chars().collect();
        let l = rucksacks.len();
        let part1:HashSet<char> = rucksacks[0..l/2].iter().cloned().collect();
        let part2:HashSet<char> = rucksacks[l/2..l].iter().cloned().collect();
        for c in part1.intersection(&part2).into_iter() {
            score += get_priority(*c)
        }
    }
    return score;
}

fn aoc2(input: &String) -> u32 {
    let mut score: u32 = 0;
    for line in input.split("\n").filter(|x| *x != "").collect::<Vec<_>>().chunks(3) {
        let mut set:HashSet<char> = line[0].chars().collect();
        for i in (1..3) {
            let cur_set:HashSet<char> = line[i].chars().collect();
            set = set.intersection(&cur_set).cloned().collect();
        }
        for c in set.iter() {
            score += get_priority(*c);
        }
    }
    return score;
}


fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{}", aoc(&contents));
    println!("{}", aoc2(&contents));
}