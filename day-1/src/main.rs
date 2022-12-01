use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let inventory = read_lines("input.txt").unwrap();

    let mut elf_calories = Vec::new();
    let mut current_elf_calories = 0;
    for item in inventory {
        match item {
            Ok(amount) => {
                if amount.is_empty() {
                    elf_calories.push(current_elf_calories);
                    current_elf_calories = 0;
                } else {
                    current_elf_calories += amount.parse::<i32>().unwrap();
                }
            }
            _ => eprintln!("failed to read a line")
        }
    }

    println!("The top elf is carrying {}", elf_calories.iter().max().unwrap());

    elf_calories.sort();

    let top_elves: i32 = elf_calories.iter()
        .rev()
        .take(3)
        .sum();

    println!("The top three elves are carrying {}", top_elves);
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
