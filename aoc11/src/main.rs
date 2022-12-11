extern crate queues;
use queues::*;
use std::fs::File;
use std::io::prelude::*;
use std::cell::RefCell;

#[derive(Debug)]
struct Operation {
    left: String,
    operator: String,
    right: String,

}

#[derive(Debug)]
struct Monkey {
    items: RefCell<Queue<u128>>,
    operation: Operation,
    divisble: u128,
    mk_true: usize,
    mk_false: usize,
    inspect_counter: u128,
}


impl Operation {
    fn new(op: &str) -> Operation {
        let tmp:Vec<&str> = op.split("=").collect();
        let ops:Vec<&str> = tmp[1].trim().split(" ").collect();
        Operation {
            left: ops[0].to_string(),
            operator: ops[1].to_string(),
            right: ops[2].to_string(),
        }
    }

    fn run(&self, old: u128, ppcm: u128) -> u128 {
        let l:u128 = if self.left == "old" { old } else { self.left.parse().expect("Nan") };
        let r:u128 = if self.right == "old" { old } else { self.right.parse().expect("Nan") };
        match self.operator.as_str() {
            "+" => if ppcm == 0 { (l + r) / 3 } else { (l+r) % ppcm },
            "*" => if ppcm == 0 { (l * r) / 3 } else { (l*r) % ppcm },
            &_ => 0,
        }
    }
}

impl Monkey {
    fn new(monkey: &str) -> Monkey {
        let tmp:Vec<&str> = monkey.split("\n").collect();
        let mut q = Queue::<u128>::new();
        for i in tmp[1].split(":").last().unwrap().trim().split(",") {
            q.add(i.trim().parse().expect("Nan"));

        }
        Monkey {
            items: RefCell::new(q),
            operation: Operation::new(tmp[2]),
            divisble: tmp[3].split(" ").last().unwrap().parse().expect("Nan"),
            mk_true: tmp[4].split(" ").last().unwrap().parse().expect("Nan"),
            mk_false: tmp[5].split(" ").last().unwrap().parse().expect("Nan"),
            inspect_counter: 0,
        }
    }

    fn inspect(&mut self, ppcm: u128) -> (u128, usize) {
        let worry:u128 = self.operation.run(self.items.borrow_mut().remove().unwrap(), ppcm);
        self.inspect_counter = self.inspect_counter + 1;
        let nb = if worry % self.divisble == 0 { self.mk_true } else { self.mk_false };
        return (worry, nb);
    }

    fn empty(&self) -> bool {
        self.items.borrow().size() == 0 
    }

    fn add(&self, worry:u128) {
        self.items.borrow_mut().add(worry);
    }
}

fn aoc(input: &String, round: usize, ppcm_compute: bool) -> u128 {
    let mut monkeys = Vec::<RefCell<Monkey>>::new(); 
    for line in input.split("\n\n").map(|x| x.trim()).filter(|&x| x != "") {
        monkeys.push(RefCell::new(Monkey::new(line)));
    }
    let ppcm:u128 = if ppcm_compute { monkeys.iter().map(|x| x.borrow().divisble).product() } else { 0 };
    

    for _ in 0..round {
        for mc in &monkeys {
            let mut m = mc.borrow_mut();
            while !m.empty() {
                let r = m.inspect(ppcm);
                monkeys[r.1].borrow_mut().add(r.0);
            }

        }
    }
    let mut inspects = monkeys.iter().map(|x| x.borrow().inspect_counter).collect::<Vec<u128>>();
    inspects.sort();
    inspects.pop().unwrap() * inspects.pop().unwrap()
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{}", aoc(&contents, 20, false));
    println!("{}", aoc(&contents, 10000, true));
}
