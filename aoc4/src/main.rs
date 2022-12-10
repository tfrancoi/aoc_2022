use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RangeAoc {
    start: u32,
    end: u32,
}

impl RangeAoc {
    pub fn new(range_str: &str) -> RangeAoc {
        let range: Vec<&str> = range_str.split("-").collect();
        RangeAoc {
            start : range[0].parse().expect("NAN"),
            end: range[1].parse().expect("NAN")
        }
    }

    pub fn contains(&self, other: &RangeAoc) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlap(&self, other: &RangeAoc) -> bool {
        self.contains(&other) ||
        other.contains(&self) ||
        (self.end <= other.end && self.end >= other.start) ||
        (self.start <= other.end && self.start >= other.start)
    }
}

fn aoc(input: &String) -> u32 {
    let mut score: u32 = 0;
    for line in input.split("\n").filter(|x| *x != "") {
        let ranges: Vec<&str> = line.split(",").collect();
        let r1 = RangeAoc::new(ranges[0]);
        let r2 = RangeAoc::new(ranges[1]);
        if r1.contains(&r2) || r2.contains(&r1) {
            score += 1;
            println!("{:?} : {:?}", r1, r2);
        }
    }
    return score;
}

fn aoc2(input: &String) -> u32 {
    let mut score: u32 = 0;
    for line in input.split("\n").filter(|x| *x != "") {
        let ranges: Vec<&str> = line.split(",").collect();
        let r1 = RangeAoc::new(ranges[0]);
        let r2 = RangeAoc::new(ranges[1]);
        if r1.overlap(&r2) {
            score += 1;
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