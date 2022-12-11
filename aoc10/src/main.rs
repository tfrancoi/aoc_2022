use std::fs::File;
use std::io::prelude::*;


fn get_nb(x: isize, cycle: usize, steps: &Vec<usize>) -> isize {
    if steps.contains(&cycle) {
        let icycle:isize = cycle as isize;
        return icycle * x;
    }
    return 0;
}

fn in_sprite(x: isize, cycle: &usize) -> String {
    let pos = ((cycle - 1) % 40) as isize;
    if pos >= x-1 && pos < x + 2 {
        return "#".to_string();
    }
    return ".".to_string();
}

fn aoc(input: &String) -> isize {
    let steps:Vec<usize> = vec![20,  60, 100, 140, 180, 220];
    let mut x:isize = 1;
    let mut cycle:usize = 1;
    let mut sum:isize = 0;
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        if line == "noop" {
            print!("{}", in_sprite(x, &cycle));
            cycle += 1;
            sum += get_nb(x, cycle, &steps);
        }
        else {
            print!("{}", in_sprite(x, &cycle));
            cycle += 1;
            if (cycle - 1) % 40 == 0 {
                print!("\n");
            }
            print!("{}", in_sprite(x, &cycle));
            sum += get_nb(x, cycle, &steps);
            cycle += 1;
            x += line.split(" ").collect::<Vec<&str>>()[1].parse::<isize>().expect("Nan");
            sum += get_nb(x, cycle, &steps);
        }

        if (cycle - 1) % 40 == 0 {
            print!("\n");
        }
    }
    print!("\n");
    sum
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{}", aoc(&contents));
}