use std::fs;

fn main() {
    let content = fs::read_to_string("./data").expect("could not read");
    let mut val: i32 = 50;
    let mut count = 0;
    for line in content.split("\n") {
        if line.len() < 1 {
            continue;
        };
        let (dir, numstr) = line.split_at(1);
        let mut num: i32 = numstr.parse().unwrap();
        let s = match dir {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };

        while num > 0 {
            val += s;
            if val == 0 {
                count += 1;
            }
            if val < 0 {
                val = 99;
            }
            if val > 99 {
                val = 0;
                count += 1;
            }
            num -= 1;
        }
    }
    print!("{}", count);
}
