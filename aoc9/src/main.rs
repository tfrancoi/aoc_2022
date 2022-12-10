use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use num::signum;

fn is_adjecent(head:&(isize, isize), tail:&(isize, isize)) -> bool {
    (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2
}

fn move_tail(head:&(isize, isize), tail:(isize, isize)) -> (isize, isize) {
    return (tail.0 + signum(head.0 - tail.0), tail.1 + signum(head.1 - tail.1))
}

fn aoc_internal(input: &String, size: usize) -> usize {
    let mut tail_pos:HashSet<(isize, isize)> = HashSet::new();
    let mut knots:Vec<(isize, isize)> = (0..size).map(|_| (0,0)).collect();
    tail_pos.insert(knots[size-1].clone());
    for line in input.split("\n").filter(|&x| x != "") {
        let command = line.split(" ").collect::<Vec<&str>>();
        let qty:usize = command[1].parse().expect("Nan");
        let moved:(isize, isize) = match command[0] {
            "R" => ( 1,  0),
            "L" => (-1,  0),
            "D" => ( 0,  1),
            "U" => ( 0, -1),
            ___ => ( 0,  0),
        };
        for _ in 0..qty {
            knots[0] = (knots[0].0 + moved.0, knots[0].1 + moved.1);
            for i in 1..knots.len() {
                if !is_adjecent(&knots[i-1], &knots[i]) {
                    knots[i] = move_tail(&knots[i-1], knots[i]);
                } 
            }
            tail_pos.insert(knots[size-1].clone());
        }
    }
    tail_pos.len()
}

fn aoc(input: &String) -> (usize, usize) {
    return (aoc_internal(input, 2), aoc_internal(input, 10));
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
