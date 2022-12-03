#![feature(array_chunks)]

use common;
use std::collections::HashSet;

fn main() {
    let rucksacks: Vec<(HashSet<char>, HashSet<char>)> = common::read_lines("input.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let (first, second) = line.split_at(line.len() / 2);
            (
                HashSet::from_iter(first.chars()),
                HashSet::from_iter(second.chars()),
            )
        })
        .collect();

    let priority_score: i32 = rucksacks
        .iter()
        .map(|(first, second)| {
            first
                .intersection(second)
                .into_iter()
                .map(|ch| to_value(*ch as u8))
                .sum::<i32>()
        })
        .sum();

    println!("priority score {}", priority_score);

    let badge_score: i32 = rucksacks
        .array_chunks::<3>()
        .map(|group| {
            let elf_badge = group
                .iter()
                .map(|(first, second)| HashSet::from_iter(first.union(&second).map(|ch| *ch)))
                .into_iter()
                .reduce(|acc: HashSet<char>, item| {
                    HashSet::from_iter(acc.intersection(&item).map(|ch| *ch))
                })
                .unwrap();

            elf_badge.iter().map(|ch| to_value(*ch as u8)).sum::<i32>()
        })
        .sum();

    println!("badge score {}", badge_score);
}

fn to_value(ch: u8) -> i32 {
    (if ch.is_ascii_lowercase() {
        ch - b'a' + 1
    } else {
        ch - b'A' + 27
    }) as i32
}
