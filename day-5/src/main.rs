#![feature(iter_next_chunk)]

use common::read_lines;

fn main() {
    let mut stacks = load_stacks();
    let moves = load_moves();

    for the_move in &moves {
        for _ in 0..the_move.0 {
            let tmp = stacks[the_move.1].pop().unwrap();
            stacks[the_move.2].push(tmp);
        }
    }

    println!("part 1 {}", top_crates(&stacks));

    let mut stacks = load_stacks();

    for the_move in &moves {
        let from_stack = stacks[the_move.1].clone();
        let (keep, to_move) = from_stack.split_at(from_stack.len() - the_move.0);

        stacks[the_move.1] = Vec::from(keep);
        stacks[the_move.2].extend_from_slice(to_move.clone());
    }

    println!("part 2 {}", top_crates(&stacks));
}

fn top_crates(stacks: &Vec<Vec<u8>>) -> String {
    String::from_utf8(
        stacks
            .iter()
            .map(|stack| stack.last().unwrap().clone())
            .collect(),
    )
    .unwrap()
}

fn load_stacks() -> Vec<Vec<u8>> {
    let mut stacks: Vec<Vec<u8>> = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in read_lines("input-stacks.txt").unwrap() {
        let line = line.unwrap();

        let mut line_chars = line.chars();

        for i in 0..9 {
            if let Ok(chunk) = line_chars.next_chunk::<3>() {
                if chunk[0] == '[' {
                    stacks[i].insert(0, chunk[1] as u8)
                }
            }
            line_chars.next();
        }
    }

    return stacks;
}

fn load_moves() -> Vec<(usize, usize, usize)> {
    read_lines("input-moves.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" ").collect();
            (
                parts[1].parse::<usize>().unwrap(),
                parts[3].parse::<usize>().unwrap() - 1,
                parts[5].parse::<usize>().unwrap() - 1,
            )
        })
        .collect()
}
