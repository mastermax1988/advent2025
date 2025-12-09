use std::{
    collections::{HashSet, VecDeque},
    fs::{self, DirBuilder},
    os::unix::raw::pid_t,
    u128,
};
#[derive(Debug)]
struct Junctionbox {
    x: i64,
    y: i64,
    z: i64,
    index: usize,
    entdeckt: bool,
}

impl Junctionbox {
    fn new(x: i64, y: i64, z: i64, index: usize) -> Self {
        Self {
            x,
            y,
            z,
            index,
            entdeckt: false,
        }
    }
    fn dist(&self, b: &Junctionbox) -> u128 {
        (i64::pow(self.x - b.x, 2) + i64::pow(self.y - b.y, 2) + i64::pow(self.z - b.z, 2))
            .try_into()
            .unwrap()
    }
}

fn main() {
    let content = fs::read_to_string("./input").expect("could not read");
    let mut junctionboxes: Vec<Junctionbox> = Vec::new();

    let mut index: usize = 0;
    for line in content.lines() {
        let mut k = line.split(",").map(|e| e.parse::<i64>().unwrap());
        junctionboxes.push(Junctionbox::new(
            k.next().unwrap(),
            k.next().unwrap(),
            k.next().unwrap(),
            index,
        ));
        index += 1;
    }

    let mut dists: Vec<Vec<u128>> = Vec::new();
    let mut matrix: Vec<Vec<bool>> = Vec::new(); //adjazenzmatrix
    for row in 0..junctionboxes.len() {
        let mut r: Vec<u128> = Vec::new();
        let mut m: Vec<bool> = Vec::new();
        for _ in 0..=row {
            r.push(0);
            m.push(false);
        }
        for col in row + 1..junctionboxes.len() {
            let a = &junctionboxes[row];
            let b = &junctionboxes[col];
            r.push(a.dist(&b));
            m.push(false);
        }
        dists.push(r);
        matrix.push(m);
    }
    let mut knotenliste: Vec<usize> = Vec::new();
    for _ in 0..1000 {
        let cord = get_min_coord(&mut dists);
        matrix[cord.0][cord.1] = true;
        matrix[cord.1][cord.0] = true;
        if !knotenliste.contains(&junctionboxes[cord.0].index) {
            knotenliste.push(junctionboxes[cord.0].index);
        }
        if !knotenliste.contains(&junctionboxes[cord.1].index) {
            knotenliste.push(junctionboxes[cord.1].index);
        }
    }
    let mut circuitlength: Vec<i64> = Vec::new();

    for k in knotenliste {
        if junctionboxes[k].entdeckt {
            continue;
        }
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut len = 0;
        queue.push_back(k);
        while queue.len() > 0 {
            let e: usize = queue.pop_front().unwrap();
            len += 1;
            for i in 0..matrix.len() {
                if !junctionboxes[i].entdeckt && matrix[e][i] {
                    junctionboxes[i].entdeckt = true;
                    queue.push_back(i);
                }
            }
        }
        circuitlength.push(len - 1);
    }
    circuitlength.sort_by(|a, b| b.cmp(a));
    let mut mul = 1;
    for i in 0..3 {
        mul *= circuitlength[i];
    }
    println!("{}", mul);
}

fn get_min_coord(dists: &mut Vec<Vec<u128>>) -> (usize, usize) {
    let mut min = u128::MAX;
    let mut irow = 0;
    let mut icol = 0;
    for row in 0..dists.len() {
        for col in row + 1..dists.len() {
            if dists[row][col] < min && dists[row][col] != 0 {
                min = dists[row][col];
                irow = row;
                icol = col;
            }
        }
    }
    dists[irow][icol] = 0;
    (irow, icol)
}
