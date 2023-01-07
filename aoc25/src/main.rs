use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn add(number: Vec<char>, number2: Vec<char>, indice:usize) -> Vec<char> {
    let mut res = number.clone();
    for (j, c) in number2.iter().enumerate() {
        if number[j + indice] == '0' {
            res[j + indice] = *c;
        }
        if number[j + indice] == '1' {
            match c {
                '=' => res[j + indice] = '-',
                '-' => res[j + indice] = '0',
                '0' => res[j + indice] = '1',
                '1' => res[j + indice] = '2',
                '2' => {
                    res[j + indice] = '=';
                    res = add(res, vec!['1'], j + indice + 1);
                },
                ___ => todo!(),

            }
        }
    }
    res
}

fn int_to_snafu(number: isize) -> String {
    let size = (number as f64).log(5.0).ceil() as usize;
    let mut res:Vec<char> = (0..size).map(|_x| '0').collect();
    let mut cur = number;
    let mut i = 0;
    while cur > 0 {
        match cur % 5 {
            0 => {
                res = add(res, vec!['0'], i);
            }
            1 => {
                res = add(res, vec!['1'], i);
            }
            2 => {
                res = add(res, vec!['2'], i);
            }
            3 => {
                res = add(res, vec!['=', '1'], i);
            }
            4 => {
                res = add(res, vec!['-', '1'], i);
            }
            _ => todo!()
        }
        cur = cur / 5;
        i += 1;
    }
    res.iter().rev().map(|x| x.to_string()).collect::<Vec<String>>().join("")
}

fn snafu_to_int(snafu_nb: &String) -> isize {
    let it = ['0', '1', '2', '-', '='].iter().zip([0, 1, 2, -1, -2].iter());
    let mapping:HashMap<&char, &isize> = HashMap::from_iter(it);
    let mut result = 0;
    for (i, c) in snafu_nb.chars().enumerate() {
        let offset:isize = (snafu_nb.len() - i - 1) as isize;
        result += mapping[&c] * isize::pow(5, offset as u32);
    }
    result
}

fn aoc(input: &String) -> (String, isize) {
    let mut result = 0;
    for line in input.split("\n").map(|x| x.trim()).filter(|&x| x != "") {
        result += snafu_to_int(&line.to_string());
    }
    (int_to_snafu(result), 0)
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error while reading file");
    println!("{:?}", aoc(&contents));
}