use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


fn aoc(input: &String) -> i32 {
    let mut win_map = HashMap::new();
    win_map.insert("X", "C");
    win_map.insert("Y", "A");
    win_map.insert("Z", "B");
    let mut even_map = HashMap::new();
    even_map.insert("X", "A");
    even_map.insert("Y", "B");
    even_map.insert("Z", "C");
    let mut score_map = HashMap::new();
    score_map.insert("X", 1);
    score_map.insert("Y", 2);
    score_map.insert("Z", 3);
    let mut score = 0;
    for line in input.split("\n").filter(|x| *x != "") {
        let game: Vec<&str> = line.split(" ").collect();
        score += score_map.get(game[1]).unwrap();
        if *even_map.get(game[1]).unwrap() == game[0] {
            score += 3;
        }
        if *win_map.get(game[1]).unwrap() == game[0] {
            score += 6;
        }
    }
    return score;
}

fn aoc2(input: &String) -> i32 {
    // X => Loose
    // y => Even
    
    let mut win_map = HashMap::new();
    win_map.insert("C", 1);
    win_map.insert("A", 2);
    win_map.insert("B", 3);
    let mut even_map = HashMap::new();
    even_map.insert("A", 1);
    even_map.insert("B", 2);
    even_map.insert("C", 3);
    let mut loose_map = HashMap::new();
    loose_map.insert("A", 3);
    loose_map.insert("B", 1);
    loose_map.insert("C", 2);
    let mut score = 0;
    for line in input.split("\n").filter(|x| *x != "") {
        let game: Vec<&str> = line.split(" ").collect();
        match game[1] {
            "X" => score += loose_map.get(game[0]).unwrap(),
            "Y" => score += 3 + even_map.get(game[0]).unwrap(),
            "Z" => score += 6 + win_map.get(game[0]).unwrap(),
            _ => score += 0,
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