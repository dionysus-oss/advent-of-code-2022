use common::{read_lines, Timer};

fn main() {
    let timer = Timer::start();

    let lines: Vec<String> = read_lines("input.txt")
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    let width = lines.last().unwrap().len();
    let height = lines.len();

    let forest: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| line.chars().map(|h| h as i32).collect())
        .collect();

    let mut possible_trees = 0;
    for y in 1..(width - 1) {
        for x in 1..(height - 1) {
            let h = forest[y][x];

            if forest[y][0..x].iter().any(|f| *f >= h)
                && forest[y][(x + 1)..].iter().any(|f| *f >= h)
                && forest[0..y].iter().any(|r| r[x] >= h)
                && forest[(y + 1)..].iter().any(|r| r[x] >= h)
            {
                possible_trees += 1;
            }
        }
    }

    println!("visible trees {}", width * height - possible_trees);

    let mut max_scenic_score = 0;
    for y in 1..(width - 1) {
        for x in 1..(height - 1) {
            let h = forest[y][x];

            let mut scenic_score = 1;

            scenic_score *= if let Some((pos, _)) = forest[y][0..x]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, f)| **f >= h)
            {
                pos + 1
            } else {
                x
            };

            scenic_score *= if let Some((pos, _)) = forest[y][(x + 1)..]
                .iter()
                .enumerate()
                .find(|(_, f)| **f >= h)
            {
                pos + 1
            } else {
                width - x - 1
            };

            scenic_score *= if let Some((pos, _)) = forest[0..y]
                .iter()
                .rev()
                .enumerate()
                .find(|(_, r)| r[x] >= h)
            {
                pos + 1
            } else {
                y
            };

            scenic_score *= if let Some((pos, _)) = forest[(y + 1)..]
                .iter()
                .enumerate()
                .find(|(_, r)| r[x] >= h)
            {
                pos + 1
            } else {
                height - y - 1
            };

            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    println!("max scenic score {}", max_scenic_score);

    timer.stop();
}
