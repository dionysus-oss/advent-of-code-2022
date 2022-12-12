#![feature(iter_array_chunks)]

use common::{read_lines, Timer};
use std::collections::VecDeque;

fn main() {
    let timer = Timer::start();

    let mut monkeys: Vec<Monkey> = load_monkeys();

    let reduce_worry_by_3x = Box::new(|v: u64| (v as f64 / 3.0).floor() as u64);

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while monkeys.get(m).unwrap().items.len() > 0 {
                let mut monkey = monkeys.get_mut(m).unwrap();
                monkey.inspections += 1;

                let item = monkey.items.pop_front().unwrap();
                let (throw_to, new_priority) = monkey.calc(item, reduce_worry_by_3x.clone());

                monkeys
                    .get_mut(throw_to)
                    .unwrap()
                    .items
                    .push_back(new_priority);
            }
        }
    }

    print_monkey_business(&mut monkeys);

    let mut monkeys: Vec<Monkey> = load_monkeys();

    let common_divs = monkeys.iter().map(|m| m.test_val).fold(1u64, |a, b| a * b);

    let reduce_worry_by_common_divs = Box::new(move |v: u64| v % common_divs);

    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            while monkeys.get(m).unwrap().items.len() > 0 {
                let mut monkey = monkeys.get_mut(m).unwrap();
                monkey.inspections += 1;

                let item = monkey.items.pop_front().unwrap();
                let (throw_to, new_priority) =
                    monkey.calc(item, reduce_worry_by_common_divs.clone());

                monkeys
                    .get_mut(throw_to)
                    .unwrap()
                    .items
                    .push_back(new_priority);
            }
        }
    }

    print_monkey_business(&mut monkeys);

    timer.stop();
}

fn load_monkeys() -> Vec<Monkey> {
    read_lines("input.txt")
        .unwrap()
        .array_chunks::<6>()
        .map(|chunk| {
            let items_line = chunk.get(1).unwrap().as_ref().unwrap();
            let items: VecDeque<u64> = items_line[18..]
                .split(", ")
                .map(|p| p.parse::<u64>().unwrap())
                .collect();

            let op_line = chunk.get(2).unwrap().as_ref().unwrap();
            let op = op_line[23..24].to_string();
            let op_amount: Option<u64> = op_line[25..].parse::<u64>().map_or(None, |v| Some(v));

            let test_val = chunk.get(3).unwrap().as_ref().unwrap()[21..]
                .parse::<u64>()
                .unwrap();

            let true_target: usize = chunk.get(4).unwrap().as_ref().unwrap()[29..]
                .parse::<usize>()
                .unwrap();
            let false_target: usize = chunk.get(5).unwrap().as_ref().unwrap()[30..]
                .parse::<usize>()
                .unwrap();

            Monkey {
                items,
                op,
                op_amount,
                test_val,
                true_target,
                false_target,
                inspections: 0,
            }
        })
        .collect()
}

fn print_monkey_business(monkeys: &mut Vec<Monkey>) {
    monkeys.sort_by(|m1, m2| m1.inspections.cmp(&m2.inspections));
    let top_two: Vec<usize> = monkeys
        .iter()
        .map(|m| m.inspections)
        .rev()
        .take(2)
        .collect();
    println!(
        "answer {}",
        *top_two.get(0).unwrap() * *top_two.get(1).unwrap()
    );
}

struct Monkey {
    items: VecDeque<u64>,
    op: String,
    op_amount: Option<u64>,
    test_val: u64,
    true_target: usize,
    false_target: usize,
    inspections: usize,
}

impl Monkey {
    pub fn calc(&self, old_priority: u64, reduce_worry: Box<dyn Fn(u64) -> u64>) -> (usize, u64) {
        let op_amount = match self.op_amount {
            Some(v) => v,
            None => old_priority,
        };

        let new_priority = reduce_worry(match self.op.as_ref() {
            "*" => old_priority * op_amount,
            "+" => old_priority + op_amount,
            _ => panic!("unknown operator"),
        });

        if new_priority % self.test_val == 0 {
            (self.true_target, new_priority)
        } else {
            (self.false_target, new_priority)
        }
    }
}
