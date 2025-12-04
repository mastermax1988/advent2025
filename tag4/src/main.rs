use std::{collections::VecDeque, fs};

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut split: VecDeque<VecDeque<&str>> =
        content.split("\n").map(|s| s.split("").collect()).collect();
    split.pop_back();
    for s in &mut split {
        s.pop_front();
        s.pop_back();
    }
    let len = split[0].len();
    split.push_front(vec!["."; len].into());
    split.push_back(vec!["."; len].into());

    for s in &mut split {
        s.push_front(".");
        s.push_back(".");
    }
    let mut sum = 0;
    loop {
        let m = remove_rolls(&mut split);
        sum += m;
        if m == 0 {
            break;
        }
    }
    println!("{}", sum);
}

fn remove_rolls(split: &mut VecDeque<VecDeque<&str>>) -> i32 {
    let mut num = 0;
    for i in 1..(split.len() - 1) {
        for j in 1..(split[0].len() - 1) {
            let mut count = 0;
            if split[i][j] == "@" {
                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        if x == i && y == j {
                            continue;
                        }
                        if split[x][y] == "@" || split[x][y] == "!" {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    num += 1;
                    split[i][j] = "!";
                }
            }
        }
    }
    for i in 1..(split.len() - 1) {
        for j in 1..(split[0].len() - 1) {
            if split[i][j] == "!" {
                split[i][j] = ".";
            }
        }
    }
    num
}
