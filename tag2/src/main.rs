use std::fs;

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut sum: u128 = 0;
    for range in content.split(',') {
        if range.len() < 1 {
            return;
        }
        let mut r = range.trim().split('-');
        let a: u128 = r.next().unwrap().parse().unwrap();
        let b: u128 = r.next().unwrap().parse().unwrap();
        for i in a..=b {
            let numstr = i.to_string();
            let maxlen = numstr.len() / 2;
            for size_of_fragments in 1..=maxlen {
                let numstr = i.to_string();
                if numstr.len() % size_of_fragments != 0 {
                    continue;
                }
                let mut treffer = true;
                for j in 1..(numstr.len() / size_of_fragments) {
                    if numstr[0..size_of_fragments]
                        != numstr[j * size_of_fragments..(j + 1) * size_of_fragments]
                    {
                        treffer = false;
                    }
                }
                if treffer {
                    sum += i;
                    break;
                }
            }
        }
    }
    println!("{}", sum);
}
