use common::read_lines;
use std::collections::{HashMap, HashSet};

fn main() {
    let moves: Vec<(String, i32)> = read_lines("input.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(" ");
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut move_def: HashMap<String, (i32, i32)> = HashMap::new();
    move_def.insert("U".to_string(), (0, 1));
    move_def.insert("D".to_string(), (0, -1));
    move_def.insert("L".to_string(), (-1, 0));
    move_def.insert("R".to_string(), (1, 0));

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);
    for (dir, num) in &moves {
        for _ in 0..*num {
            h_pos = add(&h_pos, move_def.get(dir.as_str()).unwrap());

            if dist(&h_pos, &t_pos) > 1.5 {
                let t_mov = mov(&h_pos, &t_pos);
                t_pos = add(&t_pos, &t_mov);
            }

            visited.insert(t_pos);
        }
    }

    println!("tail positions visited {}", visited.len());

    visited.clear();

    let mut rope: Vec<(i32, i32)> = Vec::with_capacity(10);
    for _ in 0..10 {
        rope.push((0, 0));
    }

    for (dir, num) in &moves {
        for _ in 0..*num {
            rope[0] = add(&rope.get(0).unwrap(), move_def.get(dir.as_str()).unwrap());

            for i in 0..9 {
                let l_pos = &rope.get(i).unwrap();
                let f_pos = &rope.get(i + 1).unwrap();

                if dist(&l_pos, &f_pos) > 1.5 {
                    let f_mov = mov(&l_pos, &f_pos);
                    rope[i + 1] = add(&f_pos, &f_mov);
                }
            }

            visited.insert(rope.get(9).unwrap().clone());
        }
    }

    println!("tail positions visited 2 {}", visited.len());
}

fn add(first: &(i32, i32), second: &(i32, i32)) -> (i32, i32) {
    (first.0 + second.0, first.1 + second.1)
}

fn dist(first: &(i32, i32), second: &(i32, i32)) -> f64 {
    let sum = ((first.0 - second.0) as f64).powf(2.0) + ((first.1 - second.1) as f64).powf(2.0);
    sum.sqrt()
}

fn mov(h_pos: &(i32, i32), t_pos: &(i32, i32)) -> (i32, i32) {
    let mut x = h_pos.0 - t_pos.0;
    let mut y = h_pos.1 - t_pos.1;

    if x != 0 {
        x /= x.abs();
    }

    if y != 0 {
        y /= y.abs();
    }

    (x, y)
}
