use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Rock {
    start: (usize, usize),
    end: (usize, usize),
}

impl Rock {
    fn build_rocks(line: &String) -> Vec<Rock>{
        let steps:Vec<&str> = line.split(" -> ").collect();
        let mut rocks:Vec<Rock> = Vec::new();
        for s in steps.windows(2) {
            let mut start:Vec<&str> = s[0].split(",").collect();
            let mut end:Vec<&str> = s[1].split(",").collect();
            if s[0] > s[1] {
                start = s[1].split(",").collect();
                end = s[0].split(",").collect();
            }
            rocks.push(Rock {
                start: (start[0].parse().expect(""), start[1].parse().expect("")),
                end: (end[0].parse().expect(""), end[1].parse().expect(""))
            });
        }
        return rocks;
    }

    fn collide(&self, pos: &(usize, usize)) -> bool {
        self.start.0 <= pos.0 && pos.0 <= self.end.0 && self.start.1 <= pos.1 && pos.1 <= self.end.1
    }

    fn colision(pos: &(usize, usize), rocks: &Vec<Rock>, sands: &HashSet<(usize, usize)>) -> bool {
        if sands.contains(pos) {
            return true;
        }
        for r in rocks {
            if r.collide(pos) {
                return true;
            }
        }
        return false;
    }

    fn get_boundaries(rocks: &Vec<Rock>) -> ((usize, usize), (usize, usize)) {
        (
            (rocks.iter().map(|r| r.start.0).min().unwrap(), rocks.iter().map(|r| r.start.1).min().unwrap()),
            (rocks.iter().map(|r| r.end.0).max().unwrap(), rocks.iter().map(|r| r.end.1).max().unwrap()),
        )
    }

    fn draw(rocks: &Vec<Rock>, sands: &HashSet<(usize, usize)>) {
        let (start, end) = Rock::get_boundaries(rocks);
        let offset_x = start.0 - 30;
        let size_x:usize = end.0 - start.0 + 60;
        let size_y:usize = end.1 - start.1 + 15;
        for j in 0..size_y  {
            for i in 0..size_x {
                if sands.contains(&(i + offset_x, j)) {
                    print!("o");
                }
                else if Rock::colision(&(i + offset_x, j), rocks, sands) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

fn next_move(pos: &(usize, usize), rocks: &Vec<Rock>, sands: &HashSet<(usize, usize)>) -> (usize, usize) {
    if !Rock::colision(&(pos.0, pos.1 + 1), rocks, sands) {
        return (pos.0, pos.1 + 1);
    }
    if !Rock::colision(&(pos.0 - 1, pos.1 + 1), rocks, sands) {
        return (pos.0 - 1, pos.1 + 1);
    }
    if !Rock::colision(&(pos.0 + 1, pos.1 + 1), rocks, sands) {
        return (pos.0 + 1, pos.1 + 1);
    }
    return (pos.0, pos.1);
}

fn aoc(input: &String) -> (usize, usize) {
    let mut sands:HashSet<(usize, usize)> = HashSet::new();
    let mut rocks:Vec<Rock> = Vec::new();
    for line in input.split("\n") {
        rocks.append(&mut Rock::build_rocks(&line.to_string()));
    }

    let (_, b_end) = Rock::get_boundaries(&rocks);
    let mut aoc1 = 0;
    loop {

        let mut pos = (500, 0);
        let mut next = next_move(&pos, &rocks, &sands);
        while pos != next && pos.1 <= b_end.1 {
            pos = next;
            next = next_move(&pos, &rocks, &sands);
        }
        if pos.1 > b_end.1 {
            if aoc1 == 0 {
                Rock::draw(&rocks, &sands);
                aoc1 = sands.len();
            }
        }
        sands.insert(pos);
        if pos == (500, 0) {
            Rock::draw(&rocks, &sands);
            return (aoc1, sands.len());
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
