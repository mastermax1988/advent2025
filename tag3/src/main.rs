use std::{collections::VecDeque, fs};

struct Bestval {
    val: u128,
    pos: usize,
}
fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut sum: u128 = 0;
    for line in content.trim().split("\n") {
        let mut max_num = 0;
        let mut v: VecDeque<u128> = VecDeque::new();
        //create vector of all digits
        for c in line.trim().split("") {
            let num = c.parse();
            match num {
                Ok(m) => v.push_back(m),
                _ => continue,
            }
        }
        let mut startindex = 0;
        for i in 0..12 {
            let res = give_best_val_in_range(&v, startindex, line.len() - 12 + i);
            max_num += res.val * u128::pow(10, (11 - i).try_into().unwrap());
            startindex = res.pos + 1;
        }
        sum += max_num;
    }
    println!("{}", sum);
}

fn give_best_val_in_range(s: &VecDeque<u128>, a: usize, b: usize) -> Bestval {
    let mut num = 0;
    let mut bestindex = 0;
    for i in a..=b {
        if s[i] > num {
            num = s[i];
            bestindex = i;
        }
    }
    Bestval {
        val: num,
        pos: bestindex,
    }
}
