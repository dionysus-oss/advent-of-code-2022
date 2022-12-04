use common::read_lines;

fn main() {
    // <number>-<number>,<number>-<number>
    let section_range_pairs: Vec<((i32, i32), (i32, i32))> = read_lines("input.txt")
        .unwrap()
        .map(|line| {
            let line = line.unwrap();
            let ranges: Vec<&str> = line.split(',').collect();
            (get_range(ranges[0]), get_range(ranges[1]))
        })
        .collect();

    let fully_contained_count: i32 = section_range_pairs
        .iter()
        .map(|pair| {
            if is_contained(pair.0, pair.1) || is_contained(pair.1, pair.0) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("contained count {}", fully_contained_count);

    let overlapping_count: i32 = section_range_pairs
        .iter()
        .map(|pair| if is_overlapping(pair.0, pair.1) { 1 } else { 0 })
        .sum();

    println!("overlapping count {}", overlapping_count);
}

fn get_range(input: &str) -> (i32, i32) {
    let range_ends: Vec<&str> = input.split('-').collect();
    (
        range_ends[0].parse::<i32>().unwrap(),
        range_ends[1].parse::<i32>().unwrap(),
    )
}

fn is_contained(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 >= right.0 && left.1 <= right.1
}

fn is_overlapping(left: (i32, i32), right: (i32, i32)) -> bool {
    ((left.0 >= right.0 && left.0 <= right.1) || (left.1 >= right.0 && left.1 <= right.1))
        || is_contained(right, left)
}
