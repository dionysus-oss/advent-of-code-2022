use regex::Regex;
use std::env::args;
use std::fs::read_to_string;

struct Sensor {
    sensor_pos: (i32, i32),
    nearest_beacon_pos: (i32, i32),
    sensor_radius: u32,
}

fn main() {
    let file = args().nth(1).expect("must provide an input file");

    // 10 for the sample input, 2000000 for the real input
    let line_num = args()
        .nth(2)
        .expect("provide a line number")
        .parse::<i32>()
        .unwrap();

    // 20 for the sample input, 4000000 for the real input
    let bound = args()
        .nth(3)
        .expect("provide bound")
        .parse::<i32>()
        .unwrap();

    let input: String = read_to_string(file).unwrap();

    let re: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);

    let sensors: Vec<Sensor> = re
        .captures_iter(input.as_str())
        .map(|cap| {
            let sensor_x: i32 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let sensor_y: i32 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_x: i32 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_y: i32 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();

            let out = Sensor {
                sensor_pos: (sensor_x, sensor_y),
                nearest_beacon_pos: (beacon_x, beacon_y),
                sensor_radius: sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y),
            };

            let min_x = sensor_x - out.sensor_radius as i32;
            if min_x < x_bounds.0 {
                x_bounds.0 = min_x;
            }

            let max_x = sensor_x + out.sensor_radius as i32;
            if max_x > x_bounds.1 {
                x_bounds.1 = max_x;
            }

            let min_y = sensor_y - out.sensor_radius as i32;
            if min_y < y_bounds.0 {
                y_bounds.0 = min_y;
            }

            let max_y = sensor_y + out.sensor_radius as i32;
            if max_y > y_bounds.1 {
                y_bounds.1 = max_y;
            }

            out
        })
        .collect();

    println!("x bounds {:?}, y_bounds {:?}", x_bounds, y_bounds);

    let width = x_bounds.0.abs_diff(x_bounds.1);
    let mut line = vec![true; width as usize];

    for sensor in &sensors {
        let mut current_x = x_bounds.0;
        for pos in &mut line {
            if *pos {
                if dist(
                    sensor.sensor_pos.0,
                    sensor.sensor_pos.1,
                    current_x,
                    line_num,
                ) <= sensor.sensor_radius
                {
                    *pos = false;
                }
            }
            current_x += 1;
        }
    }

    for sensor in &sensors {
        if sensor.nearest_beacon_pos.1 == line_num {
            line[(sensor.nearest_beacon_pos.0 - x_bounds.0) as usize] = true;
        }
    }

    let num_no_beacon = line.iter().filter(|x| !*x).count();
    println!("num no beacon {}", num_no_beacon);

    let mut found_solution: (i32, i32) = (-1, -1);
    'outer: for sensor in &sensors {
        // top-right side
        let mut y = sensor.sensor_pos.1 - sensor.sensor_radius as i32 - 1;
        for x in sensor.sensor_pos.0..=(sensor.sensor_pos.0 + sensor.sensor_radius as i32 + 1) {
            if is_solution((x, y), &sensors) && x > 0 && y > 0 && x < bound && y < bound {
                found_solution = (x, y);
                break 'outer;
            }
            y -= 1;
        }

        // bottom-right side
        let mut y = sensor.sensor_pos.1 + sensor.sensor_radius as i32 + 1;
        for x in sensor.sensor_pos.0..=(sensor.sensor_pos.0 + sensor.sensor_radius as i32 + 1) {
            if is_solution((x, y), &sensors) && x > 0 && y > 0 && x < bound && y < bound {
                found_solution = (x, y);
                break 'outer;
            }
            y -= 1;
        }

        // bottom-left side
        let mut y = sensor.sensor_pos.1;
        for x in (sensor.sensor_pos.0 - sensor.sensor_radius as i32 - 1)..=sensor.sensor_pos.0 {
            if is_solution((x, y), &sensors) && x > 0 && y > 0 && x < bound && y < bound {
                found_solution = (x, y);
                break 'outer;
            }
            y += 1;
        }

        // top-left side
        let mut y = sensor.sensor_pos.1;
        for x in (sensor.sensor_pos.0 - sensor.sensor_radius as i32 - 1)..=sensor.sensor_pos.0 {
            if is_solution((x, y), &sensors) && x > 0 && y > 0 && x < bound && y < bound {
                found_solution = (x, y);
                break 'outer;
            }
            y -= 1;
        }
    }

    println!("found solution {:?}", found_solution);
    println!(
        "answer {}",
        found_solution.0 as u64 * 4_000_000 + found_solution.1 as u64
    );
}

fn is_solution(point: (i32, i32), sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors {
        if dist(sensor.sensor_pos.0, sensor.sensor_pos.1, point.0, point.1) <= sensor.sensor_radius
        {
            return false;
        }
    }

    true
}

fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}
