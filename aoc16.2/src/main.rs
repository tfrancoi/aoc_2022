use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::{max, min};
use priority_queue::PriorityQueue;
use std::hash::{Hash, Hasher};

const TOTAL_TIME: isize = 26;

#[derive(Debug)]
struct Node {
    rate: isize,
    name: String,
    next: Vec<String>,
}

#[derive(Debug, Clone, Eq)]
struct Path {
    rate: isize,
    total_pressure: isize,
    timing: isize,
    opened: Vec<String>,
    current1: String,
    current2: String,
    done1: bool,
    done2: bool,
}
#[derive(Hash, PartialEq, Eq, Debug, Clone)]
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
                        format!("{}{}:", path, n),
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
        max((TOTAL_TIME - current_time - (distance * 1)) * rate, 0)
    }
}

impl Path {
    fn new(start: &str) -> Path {
        Path {
            current1: start.to_string(),
            current2: start.to_string(),
            rate: 0,
            total_pressure: 0,
            timing: 0,
            opened: Vec::new(),
            done1: false,
            done2: false,
        }
    }

    fn action(&self, act1: Action, act2: Action, nodes: &HashMap<String, Node>) -> Path{
        let mut cur = self._increase_timing();
        cur = match act1 {
            Action::Vopen => cur._open(nodes[&*cur.current1].rate, 1),
            Action::Vmove(node1) => cur._moved(&node1, 1),
            Action::Done => cur._done(1),
        };
        cur = match act2 {
            Action::Vopen => cur._open(nodes[&*cur.current2].rate, 2),
            Action::Vmove(node2) => cur._moved(&node2, 2),
            Action::Done => cur._done(2),
        };
        cur
    }

    fn _done(&self, slot: usize) -> Path {
        let done1 = if slot == 1 { true } else { self.done1 };
        let done2 = if slot == 2 { true } else { self.done2 };
        if done1 && done2 {
            Path {
                current1: self.current1.to_string(),
                current2: self.current2.to_string(),
                rate: self.rate,
                total_pressure: self.total_pressure, // + (self.rate * (TOTAL_TIME - self.timing)),
                timing: TOTAL_TIME,
                opened: self.opened.clone(),
                done1: true,
                done2: true,
    
            }
        }
        else {
            Path {
                current1: self.current1.to_string(),
                current2: self.current2.to_string(),
                rate: self.rate,
                total_pressure: self.total_pressure,
                timing: self.timing,
                opened: self.opened.clone(),
                done1: done1,
                done2: done2,
            }
        }
    }

    fn _increase_timing(&self) -> Path {
        let mut p = self.clone();
        p.timing += 1;
        p
    }

    fn _open(&self, rate: isize, slot:usize) -> Path {
        let mut opened = self.opened.clone();
        let node_to_open = match slot {
            1 => self.current1.to_string(),
            2 => self.current2.to_string(),
            _ => panic!("Invalid Slot"),
        };
        if opened.contains(&node_to_open) {
            // println!("Open an already opened, do nothing");
            return self.clone();
        }
        opened.push(node_to_open);
        Path {
            current1: self.current1.to_string(),
            current2: self.current2.to_string(),
            rate: self.rate + rate,
            total_pressure: self.total_pressure + rate * (TOTAL_TIME - self.timing),
            timing: self.timing,
            opened: opened,
            done1: self.done1,
            done2: self.done2,
        }
    }

    fn _moved(&self, node: &str, slot: usize) -> Path {
        Path {
            current1: match slot {
                1 => node.to_string(),
                _ => self.current1.to_string(),
            },
            current2: match slot {
                2 => node.to_string(),
                _ => self.current2.to_string(),
            },
            rate: self.rate,
            total_pressure: self.total_pressure,
            timing: self.timing,
            opened: self.opened.clone(),
            done1: self.done1,
            done2: self.done2,
        }
    }

    fn _nexts(&self, nodes: &HashMap<String, Node>, slot: usize, hd: &mut HashMap<String, isize>) -> HashSet<Action> {
        let current = if slot == 1 {self.current1.to_string() } else { self.current2.to_string() };
        let mut nexts:HashSet<Action> = HashSet::new();
        let d = Node::distances(current.to_string(), nodes, &self.opened, self.timing);
        for (node, (d, h, path)) in &d {
            if hd.contains_key(&*node) {
                hd.insert(node.to_string(), max(*h, hd[&*node]));
            }
            else {
                hd.insert(node.to_string(), *h);
            }
            if nodes[&*node].rate > 0 && *d < TOTAL_TIME - self.timing{
                if path.len() > 1 {
                    nexts.insert(Action::Vmove(path[0..2].to_string()));
                } else {
                    nexts.insert(Action::Vopen);
                }
            }
        }
        let current_nodes = &nodes[&*current];
        if current_nodes.rate > 0 && !self.opened.iter().any(|e| e == &*current) {
            nexts.insert(Action::Vopen);
        }

        if nexts.len() == 0 {
            nexts.insert(Action::Done);
        }
        return nexts;
    }
    
    fn nexts(&self, nodes: &HashMap<String, Node>) -> (isize, HashSet<Action>, HashSet<Action>) {
        let mut hd:HashMap<String, isize> = HashMap::new();
        let next1 = self._nexts(nodes, 1, &mut hd);
        let next2 = self._nexts(nodes, 2, &mut hd);
        let h = self.total_pressure + hd.values().sum::<isize>();
        return (h, next1, next2);
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.opened == other.opened &&
        self.timing == other.timing &&
        self.current1 == other.current1 &&
        self.current2 == other.current2
    }

}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.opened.hash(state);
        self.timing.hash(state);
        self.current1.hash(state);
        self.current2.hash(state);
    }
}


fn aoc(input: &String) -> isize {
    let mut nodes:HashMap<String, Node> = HashMap::new();
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        let s = Node::new(&line.to_string());
        nodes.insert(s.name.clone(), s);
    }

    let mut count = 0;
    let mut paths:PriorityQueue<Path, isize> = PriorityQueue::new();
    let mut final_paths:PriorityQueue<Path, isize> = PriorityQueue::new();
    paths.push(Path::new("AA"), 0);
    while paths.len() > 0 {
        let mut new_paths:PriorityQueue<Path, isize> = PriorityQueue::new();
        while paths.len() > 0 {
            let (path, _) = paths.pop().unwrap();
            let (h, acts1, acts2) = path.nexts(&nodes);
            if count % 10000 == 0 {
                println!("Timing {} Heuristique {} total pressure {} final_paths {} {}", path.timing, h, path.total_pressure, final_paths.len(), paths.len());
            }
            
            for act1 in  acts1 {
                for act2 in acts2.iter() {
                    let p = path.action(act1.clone(), (*act2).clone(), &nodes);
                    if p.done1 && p.done2 {
                        final_paths.push(p.clone(), p.total_pressure);
                    }
                    else {
                        new_paths.push(p, h);
                    }
                }
            }
            count = (count + 1) % 10000;
        }
        let size_max = min(new_paths.len(), 5000);
            
        for _ in 0..size_max {
            let (t, th) = new_paths.pop().unwrap();
            paths.push(t, th);
        }
    }
    let (_, fpressure ) = final_paths.peek().unwrap();
    return *fpressure;
}


fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}
