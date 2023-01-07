extern crate queues;
use queues::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn get_cubes(input: &String) -> HashSet<(isize, isize, isize)> {
    let mut cubes:HashSet<(isize, isize, isize)> = HashSet::new();
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        let temp:Vec<&str> = line.split(',').collect();
        cubes.insert((
            temp[0].parse::<isize>().unwrap(),
            temp[1].parse::<isize>().unwrap(),
            temp[2].parse::<isize>().unwrap(),
        ));
    }
    cubes
}

fn addjacent(cube:&(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    let mut adj:Vec<(isize, isize, isize)> = Vec::new();
    for t in vec![-1, 1] {
        adj.push((cube.0 + t, cube.1, cube.2));
        adj.push((cube.0, cube.1 + t, cube.2));
        adj.push((cube.0, cube.1, cube.2 + t));
    }
    adj
}

fn count_faces(addjacent: &Vec<(isize, isize, isize)>, cubes:&HashSet<(isize, isize, isize)>) -> usize {
    addjacent.iter().filter(|c| !cubes.contains(c)).collect::<Vec<_>>().len()
}

fn aoc(input: &String) -> usize {
    let cubes =  get_cubes(input);
    cubes.iter().map(|c| count_faces(&addjacent(c), &cubes)).sum::<usize>()
}

fn get_boundaries(cubes: &HashSet<(isize, isize, isize)>) -> ((isize, isize, isize), (isize, isize, isize)) {
    (
        (
            cubes.iter().map(|x| x.0).min().unwrap() - 1,
            cubes.iter().map(|x| x.1).min().unwrap() - 1,
            cubes.iter().map(|x| x.2).min().unwrap() - 1,
        ),
        (
            cubes.iter().map(|x| x.0).max().unwrap() + 1,
            cubes.iter().map(|x| x.1).max().unwrap() + 1,
            cubes.iter().map(|x| x.2).max().unwrap() + 1,
        ),
    )

}

fn filter_adjacent(
    cube: &(isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize)>,
    boundaries: &((isize, isize, isize), (isize, isize, isize)),
    visited: &HashSet<(isize, isize, isize)>
) -> (Vec<(isize, isize, isize)>, usize) {
    let mut faces = 0;
    let mut nexts:Vec<(isize, isize, isize)> = Vec::new();
    for c in addjacent(cube) {
        if c.0 < boundaries.0.0 || c.1 < boundaries.0.1 || c.2 < boundaries.0.2 ||
           c.0 > boundaries.1.0 || c.1 > boundaries.1.1 || c.2 > boundaries.1.2 || visited.contains(&c) {
            continue;
        }
        if cubes.contains(&c) {
            faces += 1;
            continue;
        }
        nexts.push(c);
    }
    (nexts, faces)

}

fn aoc2(input: &String) -> usize {
    let cubes = get_cubes(input);
    let mut visited:HashSet<(isize, isize, isize)> = HashSet::new();
    let mut nexts:Queue<(isize, isize, isize)> = Queue::new();
    let mut faces = 0;
    let boundaries = get_boundaries(&cubes);
    nexts.add(boundaries.0).ok();
    while nexts.size() > 0 {
        let cur = nexts.remove().unwrap();
        if visited.contains(&cur) {
            continue;
        }
        let (adj, new_faces) = filter_adjacent(&cur, &cubes, &boundaries, &visited);
        visited.insert(cur);
        for n in adj {
            nexts.add(n).ok();
        }
        faces += new_faces;
    }
    faces
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
    println!("{:?}", aoc2(&contents));
}