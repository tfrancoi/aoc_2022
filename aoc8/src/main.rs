use std::fs::File;
use std::io::prelude::*;


fn get_trees(trees: &Vec<Vec<usize>>, i: usize, j: usize) -> Vec<Vec<usize>> {
    vec![
        (0..i).map(|k| trees[j][k]).collect(), 
        (i+1..trees[j].len()).map(|k| trees[j][k]).collect(), 
        (0..j).map(|k| trees[k][i]).collect(), 
        (j+1..trees.len()).map(|k| trees[k][i]).collect(),
    ]
}

fn see_tree(high: usize, side_trees: Vec<Vec<usize>>) -> usize {
    for max in side_trees.iter().map(|t| t.iter().max()) {
        match max {
            None => return 1,
            Some(m) => if &high > m { return 1; }
        }
    }
    return 0;
}

fn seen_trees(high: usize, mut side_trees: Vec<Vec<usize>>) -> usize {
    side_trees[0].reverse();
    side_trees[2].reverse();
    let mut nb_trees:Vec<usize> = Vec::new();
    for side in side_trees {
        let mut done:bool = false;
        for (i, h) in side.iter().enumerate() {
            if h >= &high {
                nb_trees.push(i+1);
                done = true;
                break;
            }
        }
        if !done {
            nb_trees.push(side.len());
        }
    }
    return nb_trees.iter().product();
}

fn aoc(input: &String) -> (usize, usize) {
    let trees: Vec<Vec<usize>> = input.split("\n").filter(|&x| x != "").map(|l|
        l.chars().collect::<Vec<char>>().iter().map(
            |c| c.to_string().parse().unwrap()).collect::<Vec<usize>>()
    ).collect();

    (
        trees.iter().enumerate().map(|(j, v)| v.iter().enumerate().map(
            |(i, h)| see_tree(*h, get_trees(&trees, i, j))
        ).sum::<usize>()).sum(),
        trees.iter().enumerate().map(|(j, v)| v.iter().enumerate().map(
            |(i, h)| seen_trees(*h, get_trees(&trees, i, j))
        ).max().unwrap()).max().unwrap(),
    )
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
