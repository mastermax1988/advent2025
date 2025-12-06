use std::{collections::HashSet, fs, u64};

struct range {
    start: u64,
    end: u64,
}

impl range {
    fn containts(&self, val: u64) -> bool {
        if val >= self.start && val <= self.end {
            return true;
        }
        false
    }
    fn new(_start: u64, _end: u64) -> Self {
        range {
            start: _start,
            end: _end,
        }
    }
    fn count(&self) -> u64 {
        self.end - self.start + 1
    }
}
fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut ranges: Vec<range> = Vec::new();
    let mut scanning = true;
    let mut count: u64 = 0;
    let mut min: u64 = u64::MAX;
    let mut max: u64 = u64::MIN;
    for line in content.lines() {
        match scanning {
            true => {
                if line.len() == 0 {
                    scanning = false;
                    analyze_ranges(&mut ranges);
                    return;
                }
                let mut s = line.split("-");
                let a: u64 = s.next().unwrap().parse().unwrap();
                let b: u64 = s.next().unwrap().parse().unwrap();
                if a < min {
                    min = a;
                }
                if b > max {
                    max = b;
                }
                ranges.push(range::new(a, b));
            }
            false => {
                let num: u64 = line.parse().unwrap();
                if ranges_contains(&ranges, num) {
                    count += 1;
                }
            }
        }
    }
    println!("{} {}", max - min, count);
}

fn ranges_contains(ranges: &Vec<range>, num: u64) -> bool {
    for range in ranges {
        if range.containts(num) {
            return true;
        }
    }
    false
}

fn analyze_ranges(ranges: &mut Vec<range>) {
    let mut limits: Vec<u64> = Vec::new();
    for range in &mut *ranges {
        if !limits.contains(&range.start) {
            limits.push(range.start);
        }
        if !limits.contains(&range.end) {
            limits.push(range.end);
        }
    }
    let mut count: u64 = 0;

    limits.sort();
    for i in 0..limits.len() {
        if ranges_contains(&ranges, limits[i]) {
            if ranges_contains(ranges, limits[i] + 1) {
                count += limits[i + 1] - limits[i];
            } else {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
