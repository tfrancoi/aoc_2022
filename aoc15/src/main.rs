use regex::Regex;
use std::fs::File;
use std::cmp;
use std::io::prelude::*;
use std::collections::HashSet;


#[derive(Debug)]
struct Sensor {
    center:  (isize, isize),
    beacon: (isize, isize),
    len: isize,
}

impl Sensor {
    fn new(line: &String) -> Sensor {
        let re = Regex::new(r"x=(-?\d*), y=(-?\d*)").unwrap();
        let matches: Vec<regex::Captures<'_>> = re.captures_iter(line).collect();
        let center = (matches[0][1].parse::<isize>().unwrap(), matches[0][2].parse::<isize>().unwrap());
        let beacon = (matches[1][1].parse::<isize>().unwrap(), matches[1][2].parse::<isize>().unwrap());
        Sensor {
            center: center,
            beacon: beacon,
            len: (beacon.0 - center.0).abs() + (beacon.1 - center.1).abs(),
        }
    }

    fn get_range(&self, y: isize) -> (isize, isize) {
        let distance = (y - self.center.1).abs();
        let inf = self.center.0 - self.len + distance;
        let sup = self.center.0 + self.len - distance;
        if sup <= inf {
            return (0, 0);
        }
        return (inf, sup);
    }
}

fn merged_ranges(ranges: &mut Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    ranges.sort();
    let mut merged_ranges = vec![ranges.remove(0)];
    for (start, end) in ranges {
        let (p_start, p_end) = merged_ranges.pop().unwrap();
        if p_end < *start {
            merged_ranges.push((p_start, p_end));
            merged_ranges.push((*start, *end));
        } else {
            merged_ranges.push((cmp::min(p_start, *start), cmp::max(p_end, *end)));
        }
    }
    merged_ranges
}

fn aoc(input: &String) -> isize {
    let mut beacons:HashSet<(isize, isize)> = HashSet::new();
    let mut ranges:Vec<(isize, isize)> = Vec::new();
    let y = 2000000;
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        let s = Sensor::new(&line.to_string());
        let r = s.get_range(y);
        if r != (0, 0) {
            ranges.push(r);
        }
        if s.beacon.1 == y {
            beacons.insert(s.beacon);
        }

    }
    let merged_ranges = merged_ranges(&mut ranges);
    let beacons_in_range = beacons
        .iter()
        .filter(|b| merged_ranges
            .iter()
            .map(|r| b.0 >= r.0 && b.0 <= r.1)
            .reduce(|acc, x| acc || x).unwrap()
        )
        .collect::<Vec<_>>()
        .len();

    merged_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<isize>() - beacons_in_range as isize
}



fn aoc2(input: &String) -> isize {
    let mut sensors:Vec<Sensor> = Vec::new();
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        sensors.push(Sensor::new(&line.to_string()));
    }

    for y in 0..4000001 {
        let mut ranges:Vec<(isize, isize)> = Vec::new();
        for s in sensors.iter() {
            let r = s.get_range(y);
            if r != (0, 0) {
                ranges.push(r);
            }
        }
        let merged_ranges = merged_ranges(&mut ranges);
        if merged_ranges.len() > 1 {
            return (merged_ranges[0].1 + 1) * 4000000 + y
        }
    }
    return 0;
}



fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
    println!("{:?}", aoc2(&contents));
}

//check tout les point du perimètre extérieur: compte et quand tu arrive à 4 c'est ton trou

