use core::num;
use std::fs;

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let mut numbers: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    for i in 0..lines[0].len() {
        let col = lines[0].len() - 1 - i;
        let mut numstr = "".to_string();
        for row in 0..lines.len() {
            if lines[row][col] == '+' || lines[row][col] == '*' {
                numbers.push(numstr.parse().unwrap());
                numstr.clear();
                let mut e: u64 = match lines[row][col] {
                    '+' => 0,
                    _ => 1,
                };
                for n in &numbers {
                    match lines[row][col] {
                        '+' => e += n,
                        _ => e *= n,
                    }
                }
                sum += e;
                numbers.clear();
                continue;
            }
            if lines[row][col] != ' ' {
                numstr.push(lines[row][col]);
            }
        }
        match numstr.parse() {
            Ok(m) => numbers.push(m),
            _ => (),
        }
    }
    println!("{}", sum);
    return;

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
