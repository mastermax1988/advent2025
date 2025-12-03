use std::{collections::VecDeque, fs, os::unix::raw::gid_t};

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut sum: u64 = 0;
    for line in content.trim().split("\n") {
        let mut max_num = 0;
        let mut v: VecDeque<u64> = VecDeque::new();
        for c in line.trim().split("") {
            let num = c.parse();
            match num {
                Ok(m) => v.push_back(m),
                _ => continue,
            }
        }
        let mut vcopy = v.clone();
        for c in v.iter() {
            let mut num = c * 10;
            vcopy.pop_front();
            match give_max(&vcopy) {
                Some(m) => num += m,
                _ => continue,
            }
            if num > max_num {
                max_num = num;
            }
        }
        sum += max_num;
    }
    println!("{}", sum);
}

fn give_max(s: &VecDeque<u64>) -> Option<u64> {
    let mut max = 0;
    if (s.len() == 0) {
        return None;
    }
    for c in s.iter() {
        if *c > max {
            max = *c;
        }
    }
    Some(max)
}
