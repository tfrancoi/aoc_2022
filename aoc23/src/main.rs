use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cell::RefCell;
use std::{thread, time};



fn get_boundaries(positions: &HashSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
    (
        (
            positions.iter().map(|x| x.0).min().unwrap(),
            positions.iter().map(|x| x.1).min().unwrap(),
        ),
        (
            positions.iter().map(|x| x.0).max().unwrap() + 1,
            positions.iter().map(|x| x.1).max().unwrap() + 1,
        ),
    )
}

fn draw(positions: &HashSet<(isize, isize)>) {
    let (top, bottom) = get_boundaries(positions);
    let mut output = String::new();
    for j in top.1..bottom.1 {
        for i in top.0..bottom.0 {
            if positions.contains(&(i, j)) {
                output += "#";
            }
            else {
                output += ".";
            }
        }
        output += "\n";
    }
    println!("{output}");
}

fn move_north(p: &(isize, isize), positions: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    if (-1..2).all(|x| !positions.contains(&(p.0 + x, p.1 - 1))) {
        return Some((p.0, p.1 - 1))
    }
    return None;
}

fn move_south(p: &(isize, isize), positions: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    if (-1..2).all(|x| !positions.contains(&(p.0 + x, p.1 + 1))) {
        return Some((p.0, p.1 + 1))
    }
    return None;
}

fn move_west(p: &(isize, isize), positions: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    if (-1..2).all(|x| !positions.contains(&(p.0 - 1, p.1 + x))) {
        return Some((p.0 - 1, p.1))
    }
    return None;
}

fn move_east(p: &(isize, isize), positions: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    if (-1..2).all(|x| !positions.contains(&(p.0 + 1, p.1 + x))) {
        return Some((p.0 + 1, p.1))
    }
    return None;
}

fn decide_move(positions: &HashSet<(isize, isize)>, round: usize) -> HashMap<(isize, isize), RefCell<Vec<(isize, isize)>>> {
    /*
    If no other Elves are in one of those eight positions, the Elf does not do anything
    If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
    If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
    If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
    If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.

    Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions. 
    */ 
    let mut functions:Vec<fn(&(isize, isize), &HashSet<(isize, isize)>) -> Option<(isize, isize)>> = vec![
        move_north, move_south, move_west, move_east
    ];
    for _ in 0..round % 4 {
        let f = functions.remove(0);
        functions.push(f);
    }
        
    let mut moves:HashMap<(isize, isize), RefCell<Vec<(isize, isize)>>> = HashMap::new();
    for p in positions.iter() {
        if (-1..2).all(|x| (-1..2).all(|y| (x, y) == (0, 0) || !positions.contains(&(p.0 + x, p.1 + y)))) {
            continue
        }
        for nb in 0..4 {
            match functions[nb](p, positions) {
                Some(pos) => {
                    if !moves.contains_key(&pos) {
                        moves.insert(pos, RefCell::new(Vec::new()));
                    }
                    moves[&pos].borrow_mut().push(*p);
                    break;
                }
                None => {}
            }
        }
    }
    return moves;
}

fn move_elf(positions: &mut HashSet<(isize, isize)>, plan_moves: HashMap<(isize, isize), RefCell<Vec<(isize, isize)>>>) {
    for (futur_pos, value) in plan_moves {
        let current_pos = value.borrow();
        if current_pos.len() == 1 {
            positions.remove(&current_pos[0]);
            positions.insert(futur_pos);
        }
    }
}

fn aoc(input: &String) -> (isize, usize) {
    let mut aoc = 0;
    let mut positions:HashSet<(isize, isize)> = HashSet::new();
    for (j, line) in input.split("\n").map(|x| x.trim()).filter(|&x| x != "").enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                positions.insert((i as isize, j as isize));
            }
        }
    }

    let mut round = 0;
    loop {
        let pos_copy = positions.clone();
        let plan_moves = decide_move(&positions, round);

        move_elf(&mut positions, plan_moves);
        if positions == pos_copy {
            return (aoc, round + 1);
        }
        print!("\x1Bc");
        draw(&positions);

        let ten_millis = time::Duration::from_millis(10);

        thread::sleep(ten_millis);
        if round == 9 {
            let (top, bottom) = get_boundaries(&positions);
            aoc = (bottom.0 - top.0) * (bottom.1 - top.1) - positions.len() as isize
        }
        round += 1;
    } 
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}