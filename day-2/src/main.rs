extern crate core;

use common;

fn main() {
    let plays: Vec<(i8, i8)> = common::read_lines("input.txt").unwrap().map(|line| {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        ((iter.next().unwrap().as_bytes()[0] - b'A') as i8, (iter.next().unwrap().as_bytes()[0] - b'X') as i8)
    }).collect();

    let total_score: i32 = plays.iter().map(|(theirs, ours)| score(*theirs, *ours)).sum();

    println!("part 1 - {}", total_score);

    let new_score: i32 = plays.iter().map(|(theirs, ours)| pick_play(*theirs, *ours)).map(|(theirs, ours)| score(theirs, ours)).sum();

    println!("part 2 - {}", new_score);
}

fn score(theirs: i8, ours: i8) -> i32 {
    return (win_multiplier(theirs, ours) * 3 + (ours + 1)) as i32;
}

fn win_multiplier(theirs: i8, ours: i8) -> i8 {
    let diff = ours - theirs;
    (match diff { 2 => -1, -2 => 1, _ => diff }) + 1
}

fn pick_play(theirs: i8, ours: i8) -> (i8, i8) {
    (theirs, match theirs {
        0 => (ours + 2) % 3,
        2 => (ours + 1) % 3,
        _ => ours,
    })
}

