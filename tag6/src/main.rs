use std::{clone, fs};

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut elements: Vec<Vec<u64>> = Vec::new();
    let mut op: Vec<char> = Vec::new();
    for line in content.lines() {
        let mut nums: Vec<u64> = Vec::new();
        for mut s in line.split(" ") {
            s = s.trim();
            if s.len() == 0 {
                continue;
            }
            match s.parse() {
                Ok(n) => nums.push(n),
                Err(_) => op.push(s.chars().next().unwrap()),
            }
        }
        if nums.iter().count() > 0 {
            elements.push(nums);
        }
    }
    println!("{:?}", elements);
    let mut sum: u64 = 0;
    for colls in 0..elements[0].len() {
        let mut e: u64 = match op[colls] {
            '+' => 0,
            _ => 1,
        };
        for rows in 0..elements.len() {
            match op[colls] {
                '+' => e += elements[rows][colls],
                _ => e *= elements[rows][colls],
            }
        }
        sum += e;
    }
    println!("{}", sum);
}
