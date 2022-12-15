use std::fs::File;
use std::io::prelude::*;
use std::hash::{Hash, Hasher};
use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;


#[derive(Debug,Eq)]
struct Node {
    pos: (usize, usize),
    cost: usize,
    h: usize,
}

impl Node {
    fn new(pos:(usize, usize), cost:usize, summit:&(usize, usize), map_h: &HashMap<(usize, usize), usize>) -> Node {
        Node {
            pos: pos,
            cost: cost,
            // Diagonal Distance * time hight / normalization
            h: summit.0.abs_diff(pos.0) + summit.1.abs_diff(pos.1) * (map_h[summit] - map_h[&pos]) / 13,
        }
    }

    fn get_h(&self) -> usize {
        self.cost + self.h
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1 && self.cost == other.cost
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.cost.hash(state);
    }
}

fn next_move(pos: &(usize, usize), map_h: &HashMap<(usize, usize), usize>) -> Vec<(usize, usize)> {
    let cur_h = map_h[pos];
    let mut next_moves:Vec<(usize, usize)> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            let x_i = pos.0 as isize + x;
            let y_j = pos.1 as isize + y;
            if (x == 0 && y == 0) || (x != 0 && y != 0) {
                continue;
            }
            if x_i < 0 || y_j < 0 {
                continue;
            }
            let xi:usize = x_i as usize;
            let yj:usize = y_j as usize;
            if !map_h.contains_key(&(xi, yj)) {
                continue;
            }
            if cur_h as isize >= (map_h[&(xi, yj)] as isize - 1) {
                next_moves.push((xi, yj));
            }
        }
    }
    return next_moves;
}

fn find_shortest_path(summit: &(usize, usize), start_pos: (usize, usize), map_h: &HashMap<(usize, usize), usize>) -> usize {
    let start = Node::new(start_pos, 0, summit, map_h);
    let mut pq = DoublePriorityQueue::new();
    let mut closed:HashMap<(usize, usize), Node> = HashMap::new();
    let h = start.get_h();
    pq.push(start, h);
    let mut step = 0;
    while !pq.is_empty() {
        step += 1;
        let item = pq.pop_min().unwrap();
        if item.0.pos.0 == summit.0 && item.0.pos.1 == summit.1 {
            println!("Steps {step} {:?}", item);
            return item.0.cost;
        }

        for next in next_move(&item.0.pos, &map_h) {
            let previous = closed.get(&next);
            let mut explore = true;
            // this is useless for this exercices but it's a finally a good implementation
            match previous {
                None => explore = true,
                Some(x) => {
                    if x.cost > item.0.cost + 1 {
                        closed.remove(&next);
                        explore = true;
                    }
                    else { explore = false; }
                },
            };
            if explore {
                let new_node = Node::new(next, item.0.cost + 1, &summit, map_h);
                let priority = new_node.get_h();
                pq.push(new_node, priority );
            }
        }
        closed.insert(item.0.pos, item.0);
    }
    return usize::MAX;

}

fn aoc(input: &String) -> (usize, usize) {
    let mut map_h = HashMap::<(usize, usize), usize>::new();
    let mut start = (0, 0);
    let mut summit = (0, 0);
    for (j, line) in input.split("\n").filter(|&x| x != "").enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
            }
            if c == 'E' {
                summit = (i, j);
            }
            map_h.insert((i, j), match c {
                'S' => 1,
                'E' => 26,
                 x => u32::from(x) as usize - 96,
            });
        }
    }

    (
        find_shortest_path(&summit, start, &map_h),
        map_h.keys().filter(|pos| map_h[pos] == 1).map(
            |s| find_shortest_path(&summit, *s, &map_h)
        ).min().unwrap(),
    )

}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}