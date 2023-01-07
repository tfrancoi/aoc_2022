use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;
use queues::*;
use priority_queue::PriorityQueue;

const TOTAL_TIME: isize = 30;

#[derive(Debug)]
struct Node {
    rate: isize,
    name: String,
    next: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    rate: isize,
    total_pressure: isize,
    timing: isize,
    opened: Vec<String>,
    current: String,
}
#[derive(Hash, PartialEq, Eq, Debug)]
enum Action {
    Vmove(String),
    Vopen,
    Done,
}

impl Node {
    fn new(input: &String) -> Node  {
        let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d*); tunnels? leads? to valves? (.*)").unwrap();
        let matches: Vec<regex::Captures<'_>> = re.captures_iter(input).collect();
        Node {
            name: matches[0][1].to_string(),
            rate: matches[0][2].parse::<isize>().unwrap(),
            next: matches[0][3].split(", ").map(|x| x.to_string()).collect(),

        }
    }

    fn distances(current_node: String, nodes: &HashMap<String, Node>, opened: &Vec<String>, current_time: isize) 
                -> HashMap<String, (isize, isize, String)> {
        let mut nexts:Vec<(String, isize, isize, String)> = Vec::new(); 
        let mut distances:HashMap<String, (isize, isize, String)> = HashMap::new();
        let start_node = &nodes[&*current_node];
        nexts.push((current_node.to_string(), 0, Node::expected_release(current_time, start_node.rate, 0, current_node.to_string(), opened), "".to_string()));
        let max_distance = 30 - current_time;
        while nexts.len() > 0 {
            let (node, distance, h, path) = nexts.remove(0);
            if distance >= max_distance {
                return distances;
            }
            if !distances.contains_key(&*node) {
                distances.insert(node.to_string(), (distance, h, path.to_string()));
                for n in &nodes.get(&*node).unwrap().next {
                    let n_ref = &nodes[&*n];
                    nexts.push((
                        n.to_string(),
                        distance + 1,
                        Node::expected_release(current_time, n_ref.rate, distance + 1,  n.to_string(), opened),
                        format!("{}{}:", path, n)
                    ));
                }
            }
        }
        return distances;
    }

    fn expected_release(current_time: isize, rate: isize, distance: isize, current_node: String, opened: &Vec<String>) -> isize {
        if opened.iter().any(|e| e == &*current_node) {
            return 0;
        }
        max((TOTAL_TIME - current_time - (distance * 2) - 1) * rate, 0)
    }
}

impl Path {
    fn new(start: &str) -> Path {
        Path {
            current: start.to_string(),
            rate: 0,
            total_pressure: 0,
            timing: 0,
            opened: Vec::new(),
        }
    }

    fn done(&self) -> Path {
        Path {
            current: self.current.to_string(),
            rate: self.rate,
            total_pressure: self.total_pressure + (self.rate * (TOTAL_TIME - self.timing)),
            timing: TOTAL_TIME,
            opened: self.opened.clone(),
        }
    }

    fn open(&self, rate: isize) -> Path {
        let mut opened = self.opened.clone();
        opened.push(self.current.to_string());
        Path {
            current: self.current.to_string(),
            rate: self.rate + rate,
            total_pressure: self.total_pressure + self.rate,
            timing: self.timing + 1,
            opened: opened,
        }
    }

    fn moved(&self, node: &str) -> Path {
        Path {
            current: node.to_string(),
            rate: self.rate,
            total_pressure: self.total_pressure + self.rate,
            timing: self.timing + 1,
            opened: self.opened.clone(),
        }
    }

    fn next(&self, nodes: &HashMap<String, Node>) -> Action {
        let d = Node::distances(self.current.to_string(), nodes, &self.opened, self.timing);
        println!("Distance {:?}", d);
        println!("Opened {:?}", self.opened);
        let mut max:isize = 0;
        let mut node_max:String = "".to_string();
        for (node, (_d, h, _path)) in &d {
            if *h > max {
                max = *h;
                node_max = node.to_string();
            }
        }
        if max == 0 {
            println!("Done {}", self.current);
            return Action::Done;
        }
        let current = &nodes[&*self.current];
        println!("Node Max {node_max}");
        let target = &nodes[&*node_max];
        if current.rate * (TOTAL_TIME - self.timing - 1) > target.rate && !self.opened.iter().any(|e| e == &*self.current) {
            println!("Open current {}", self.current);
            return Action::Vopen;
        }
        if current.name == target.name {
            return Action::Vopen;
        }
        else {
            let path = &d[&*node_max].2;
            println!("Move to {path}");
            return Action::Vmove(path[0..2].to_string());
        }
    }

    fn nexts(&self, nodes: &HashMap<String, Node>) -> (isize, HashSet<Action>) {
        let mut nexts:HashSet<Action> = HashSet::new();
        let mut heu = self.total_pressure + self.rate * (TOTAL_TIME - self.timing);
        let d = Node::distances(self.current.to_string(), nodes, &self.opened, self.timing);
        //println!("Distance {:?} {:?}", d, self);
        // println!("Opened {:?}", self.opened);
        for (node, (_d, h, path)) in &d {
            if *h > 0 {
                heu += *h;
                if path.len() > 1 {
                    nexts.insert(Action::Vmove(path[0..2].to_string()));
                } else {
                    nexts.insert(Action::Vopen);
                }
            }
        }
        let current = &nodes[&*self.current];
        if current.rate > 0 && !self.opened.iter().any(|e| e == &*self.current) {
            nexts.insert(Action::Vopen);
        }

        if nexts.len() == 0 {
            println!("Done {}", self.current);
            nexts.insert(Action::Done);
        }
        return (heu, nexts);
    }
}

fn aoc(input: &String) -> isize {
    let mut nodes:HashMap<String, Node> = HashMap::new();
    let mut paths:PriorityQueue<Path, isize> = PriorityQueue::new();
    let mut final_paths:PriorityQueue<Path, isize> = PriorityQueue::new();
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        let s = Node::new(&line.to_string());
        nodes.insert(s.name.clone(), s);
    }
    let mut path = Path::new("AA");
    while path.timing < 30 {
        match path.next(&nodes) {
            Action::Vopen => {
                let rate = nodes[&*path.current].rate;
                path = path.open(rate);
            }
            Action::Vmove(node) => {
                path = path.moved(&node);
            }
            Action::Done => {
                path = path.done();
            }
        }
    }
    final_paths.push(path.clone(), path.total_pressure);


    paths.push(Path::new("AA"), 0);
    while paths.len() > 0 {
        let (path, _) = paths.pop().unwrap();
        let (h, acts) = path.nexts(&nodes);
        //println!("TIming {} Heuristique {} total pressure {} final_paths {}", path.timing, h, path.total_pressure, final_paths.len());
        if final_paths.len() > 0 {
            let (p, pressure ) = final_paths.peek().unwrap();
            // println!("Pressure {}, heuristique {}", pressure, h);
            if pressure >= &h {
                continue;
            }
        }
        for act in  acts{
            match act {
                Action::Vopen => {
                    let rate = nodes[&*path.current].rate;
                    paths.push(path.open(rate), h);
                }
                Action::Vmove(node) => {
                    paths.push(path.moved(&node), h);
                }
                Action::Done => {
                    let f = path.done();
                    final_paths.push(f.clone(), f.total_pressure);
                }
            }
        }
    }
    let (fp, fpressure ) = final_paths.peek().unwrap();
    *fpressure
}


fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
