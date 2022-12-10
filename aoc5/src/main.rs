use std::fs::File;
use std::io::prelude::*;

struct MoveLine {
    nb: usize,
    from: usize,
    to: usize
}

impl MoveLine {
    fn new(line: &str) -> MoveLine {
        let move_details:Vec<&str> =  line.split(" ").collect();
        MoveLine {
            nb: move_details[1].parse::<usize>().expect("Nan"),
            from: move_details[3].parse::<usize>().expect("Nan") - 1,
            to: move_details[5].parse::<usize>().expect("Nan") -1,
        }
    }
}

fn setup(setup: &str) -> Vec<Vec<char>> {
    let mut setup:Vec<&str> = setup.split("\n").collect();
    let mut pos:Vec<usize> = Vec::new();
    for (i, &c) in setup.pop().unwrap().chars().collect::<Vec<_>>().iter().enumerate() {
        if c != ' ' {
            pos.push(i);
        }
    }
    let mut stacks:Vec<Vec<char>> = Vec::new();
    for _ in pos.iter() {
        stacks.push(Vec::new());
    }
    for _ in 0..setup.len() {
        let crates:Vec<char> = setup.pop().unwrap().chars().collect();
        for (i, &p) in pos.iter().enumerate() {
            if crates[p] != ' ' { stacks[i].push(crates[p]); }
        }
    }
    stacks
}

fn aoc(input: &String) -> String {
    let temp:Vec<&str> = input.split("\n\n").collect();
    let mut stacks = setup(temp[0]);

    let moves:Vec<&str> = temp[1].split("\n").filter(|x| *x != "").collect();
    for move_line in moves {
        let m = MoveLine::new(move_line);
        for _ in 0..m.nb {
            let temp = stacks[m.from].pop().unwrap();
            stacks[m.to].push(temp);
        }
    }
    stacks.iter().map(|x| x.last().unwrap()).fold(String::from(""), |acc, x| acc + &x.to_string())

}

fn aoc2(input: &String) -> String {
    let temp:Vec<&str> = input.split("\n\n").collect();
    let mut stacks = setup(temp[0]);

    let moves:Vec<&str> = temp[1].split("\n").filter(|x| *x != "").collect();
    for move_line in moves {
        let m = MoveLine::new(move_line);
        let offset:usize = stacks[m.from].len() - m.nb;
        let mut temp:Vec<char> = stacks[m.from].split_off(offset);
        stacks[m.to].append(&mut temp);
    }
    stacks.iter().map(|x| x.last().unwrap()).fold(String::from(""), |acc, x| acc + &x.to_string())

}


fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{}", aoc(&contents));
    println!("{}", aoc2(&contents));
}