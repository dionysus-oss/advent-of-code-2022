use common::read_lines;
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::path::Path;

type Node = (usize, usize);

fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let file = args().nth(1).unwrap();

    let grid: HashMap<(usize, usize), u8> = read_lines(file)
        .unwrap()
        .enumerate()
        .flat_map(|(row_number, line)| {
            let line = line.unwrap();
            println!("{}", line);
            let chars = line.chars();
            chars
                .enumerate()
                .map(|(col_number, ch)| match ch {
                    'S' => {
                        start = (row_number, col_number);
                        (start, 0)
                    }
                    'E' => {
                        end = (row_number, col_number);
                        (end, 25)
                    }
                    _ => ((row_number, col_number), ch as u8 - b'a'),
                })
                .collect::<Vec<((usize, usize), u8)>>()
                .into_iter()
        })
        .collect();

    println!("start at {:?}", start);
    println!("end at {:?}", end);
    println!();

    println!("path length = {}", a_star_search(&grid, start, end, h));
}

fn a_star_search(
    grid: &HashMap<(usize, usize), u8>,
    start: Node,
    end: Node,
    h: fn(Node, Node) -> usize,
) -> usize {
    // Nodes still to consider
    let mut open_set: HashSet<Node> = HashSet::new();
    open_set.insert(start);

    // Best paths from one node to another
    // let mut trace: HashMap<Node, Node> = HashMap::new();

    let mut past_cost: HashMap<Node, usize> = HashMap::new();
    for n in grid.keys() {
        past_cost.insert(*n, usize::MAX);
    }
    past_cost.insert(start, 0);

    let mut future_cost: HashMap<Node, usize> = HashMap::new();
    future_cost.insert(start, h(start, end));

    while !open_set.is_empty() {
        let mut min_future_cost = usize::MAX;
        let mut current: Node = (0, 0);
        for open_node in &open_set {
            if future_cost[open_node] < min_future_cost {
                min_future_cost = future_cost[open_node];
                current = *open_node;
            }
        }

        println!("considering {:?}", current);
        println!("from open set {:?}", open_set);

        if current == end {
            println!("finished at {:?}", end);
            return *past_cost.get(&end).unwrap();
        }

        open_set.remove(&current);

        let mut neighbors: Vec<Node> = Vec::with_capacity(4);
        let current_height = grid.get(&current).unwrap();

        if current.1 > 0 {
            let left = (current.0, current.1 - 1);
            if valid_move(grid, current_height, &left) {
                println!(
                    "left is - {:?} with height {}",
                    left,
                    (grid.get(&left).unwrap() + b'a') as char
                );
                neighbors.push(left);
            }
        }

        let right = (current.0, current.1 + 1);
        if valid_move(grid, current_height, &right) {
            println!(
                "right is - {:?} with height {}",
                right,
                (grid.get(&right).unwrap() + b'a') as char
            );
            neighbors.push(right);
        }

        if current.0 > 0 {
            let up = (current.0 - 1, current.1);
            if valid_move(grid, current_height, &up) {
                println!(
                    "up is - {:?} with height {}",
                    up,
                    (grid.get(&up).unwrap() + b'a') as char
                );
                neighbors.push(up);
            }
        }

        let down = (current.0 + 1, current.1);
        if valid_move(grid, current_height, &down) {
            println!(
                "down is - {:?} with height {}",
                down,
                (grid.get(&down).unwrap() + b'a') as char
            );
            neighbors.push(down);
        }

        for pn in &neighbors {
            println!(
                "possible neighbor - {:?} with height {}",
                pn,
                (grid.get(pn).unwrap() + b'a') as char
            );
        }

        for neighbor in neighbors {
            let maybe_new_past_cost = past_cost.get(&current).unwrap() + 1;

            println!(
                "maybe new {}, current {}",
                maybe_new_past_cost,
                *past_cost.get(&neighbor).unwrap()
            );
            if maybe_new_past_cost < *past_cost.get(&neighbor).unwrap() {
                //trace.insert(neighbor, current);
                past_cost.insert(neighbor, maybe_new_past_cost);
                future_cost.insert(neighbor, maybe_new_past_cost + h(neighbor, end));

                if open_set.get(&neighbor) == None {
                    println!("push new neighbor {:?}", neighbor);
                    open_set.insert(neighbor);
                }
            }
        }

        println!();
    }

    panic!("No path found!");
}

fn valid_move(grid: &HashMap<(usize, usize), u8>, current_height: &u8, move_to: &Node) -> bool {
    grid.contains_key(move_to) && *grid.get(move_to).unwrap() <= current_height + 1
}

fn h(p1: Node, p2: Node) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}
