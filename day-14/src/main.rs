use common::{read_lines, Timer};
use std::cmp::{max, min};
use std::env::args;
use std::vec;

#[derive(Clone, Debug, PartialEq)]
enum CavePoint {
    Air,
    Sand,
    Rock,
}

fn main() {
    let timer = Timer::start();

    let file = args().nth(1).expect("must provide a file as input");

    let (mut cave, y_bound) = load_cave(file);

    let mut done = false;
    let mut total_sand_part_1 = 0;
    let mut total_sand = 0;
    loop {
        let mut sand_pos: (usize, usize) = (500, 0);

        loop {
            if sand_pos.1 >= y_bound && total_sand_part_1 == 0 {
                total_sand_part_1 = total_sand;
            }

            let down = (sand_pos.0, sand_pos.1 + 1);
            if cave[down.1][down.0] == CavePoint::Air {
                sand_pos = down;
                continue;
            }

            let down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            if cave[down_left.1][down_left.0] == CavePoint::Air {
                sand_pos = down_left;
                continue;
            }

            let down_right = (sand_pos.0 + 1, sand_pos.1 + 1);
            if cave[down_right.1][down_right.0] == CavePoint::Air {
                sand_pos = down_right;
                continue;
            }

            cave[sand_pos.1][sand_pos.0] = CavePoint::Sand;
            total_sand += 1;
            if sand_pos == (500, 0) {
                done = true;
            }
            break;
        }

        if done {
            break;
        }
    }

    println!("amount of sand part 1 - {}", total_sand_part_1);
    println!("amount of sand part 2 - {}", total_sand);

    timer.stop();
}

fn load_cave(file: String) -> (Vec<Vec<CavePoint>>, usize) {
    let mut cave: Vec<Vec<CavePoint>> = Vec::with_capacity(200);
    for _ in 0..200 {
        cave.push(vec![CavePoint::Air; 1000]);
    }

    let mut y_bound = 0;

    for line in read_lines(file).unwrap() {
        let mut previous: Option<(usize, usize)> = None;
        let line = line.unwrap();
        let mut lines = line.split(" -> ");
        while let Some(current) = lines.next() {
            if previous == None {
                let prev = parse_coords(current);
                previous = Some(prev);
                if prev.1 > y_bound {
                    y_bound = prev.1;
                }
                continue;
            }

            let prev = previous.unwrap();
            let current = parse_coords(current);

            if current.1 > y_bound {
                y_bound = current.1;
            }

            if prev.0 == current.0 {
                for i in min(prev.1, current.1)..=max(prev.1, current.1) {
                    cave[i][prev.0] = CavePoint::Rock;
                }
            } else {
                for i in min(prev.0, current.0)..=max(prev.0, current.0) {
                    cave[prev.1][i] = CavePoint::Rock;
                }
            }

            previous = Some(current);
        }
    }

    cave.truncate(y_bound + 1);
    cave.push(vec![CavePoint::Air; 1000]);
    cave.push(vec![CavePoint::Rock; 1000]);

    return (cave, y_bound);
}

fn parse_coords(coords: &str) -> (usize, usize) {
    let mut parts = coords.split(",");
    (
        parts.next().unwrap().parse::<usize>().unwrap(),
        parts.next().unwrap().parse::<usize>().unwrap(),
    )
}
