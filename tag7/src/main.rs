use std::fs;

fn main() {
    let content = fs::read_to_string("./input")
        .expect("could not read")
        .replace("S", "|");
    let mut lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let mut count: Vec<Vec<u128>> = content
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '|' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let mut split: u128 = 0;
    for row in 1..lines.len() {
        for col in 0..lines[0].len() {
            match lines[row][col] {
                '^' => {
                    if lines[row - 1][col] == '|' {
                        lines[row][col - 1] = '|';
                        lines[row][col + 1] = '|';
                        count[row][col - 1] += count[row - 1][col];
                        count[row][col + 1] += count[row - 1][col];
                    }
                }
                '.' => {
                    if lines[row - 1][col] == '|' {
                        lines[row][col] = '|';
                        count[row][col] += count[row - 1][col];
                    }
                }
                _ => count[row][col] += count[row - 1][col],
            }
        }
    }
    for n in count[count.len() - 1].iter() {
        split += n;
    }
    println!("{}", split);
}
