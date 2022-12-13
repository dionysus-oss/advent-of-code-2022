#![feature(iter_array_chunks)]

use common::{read_lines, Timer};
use std::cmp::Ordering;
use std::env::args;
use std::vec::IntoIter;

#[derive(Debug)]
enum ParsedItem<N, D> {
    Empty,
    Num(N),
    Delayed(D),
}

type Item = ParsedItem<u8, String>;

fn main() {
    let timer = Timer::start();

    let file = args().nth(1).expect("must provide a file as input");
    let mut packets: Vec<String> = read_lines(file.clone())
        .unwrap()
        .filter_map(|line| {
            let line = line.unwrap();
            match line.as_ref() {
                "" => None,
                _ => Some(line),
            }
        })
        .collect();

    let result: usize = packets
        .iter()
        .array_chunks::<2>()
        .enumerate()
        .map(|(index, [left, right])| {
            if in_order(left.to_string(), right.to_string()).unwrap() {
                index + 1
            } else {
                0
            }
        })
        .sum();

    println!("part 1 - {}", result);

    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    packets.sort_by(|left, right| {
        if in_order(left.to_string(), right.to_string()).unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let part_2_result: usize = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| packet == "[[2]]" || packet == "[[6]]")
        .map(|(index, _)| index + 1)
        .product();

    println!("part 2 - {}", part_2_result);

    timer.stop();
}

fn in_order(left: String, right: String) -> Option<bool> {
    let left_parsed = parse(left);
    let right_parsed = parse(right);

    in_order_from_parsed(left_parsed, right_parsed)
}

fn in_order_from_parsed(mut left: IntoIter<Item>, mut right: IntoIter<Item>) -> Option<bool> {
    loop {
        match (left.next(), right.next()) {
            (Some(left_item), Some(right_item)) => match (left_item, right_item) {
                (ParsedItem::Num(l), ParsedItem::Num(r)) => {
                    if l != r {
                        return Some(l < r);
                    }
                }
                (ParsedItem::Num(l), ParsedItem::Delayed(r)) => {
                    let sub = in_order_from_parsed(vec![ParsedItem::Num(l)].into_iter(), parse(r));
                    if sub.is_some() {
                        return sub;
                    }
                }
                (ParsedItem::Delayed(l), ParsedItem::Num(r)) => {
                    let sub = in_order_from_parsed(parse(l), vec![ParsedItem::Num(r)].into_iter());
                    if sub.is_some() {
                        return sub;
                    }
                }
                (ParsedItem::Delayed(l), ParsedItem::Delayed(r)) => {
                    let sub = in_order_from_parsed(parse(l), parse(r));
                    if sub.is_some() {
                        return sub;
                    }
                }
                (ParsedItem::Empty, ParsedItem::Num(_)) => return Some(true),
                (ParsedItem::Num(_), ParsedItem::Empty) => return Some(false),
                (ParsedItem::Delayed(_), ParsedItem::Empty) => {
                    return Some(false);
                }
                (ParsedItem::Empty, ParsedItem::Delayed(_)) => {
                    return Some(true);
                }
                (ParsedItem::Empty, ParsedItem::Empty) => {}
            },
            (None, Some(_)) => return Some(true),
            (Some(_), None) => return Some(false),
            (None, None) => {
                break;
            }
        }
    }

    None
}

fn parse(input: String) -> IntoIter<ParsedItem<u8, String>> {
    let mut chars = input.chars();

    let mut out: Vec<ParsedItem<u8, String>> = Vec::new();

    if input.starts_with('[') {
        chars.next();
    }

    while let Some(next) = chars.next() {
        match next {
            '[' => {
                let mut buf = Vec::new();
                buf.push('[' as u8);
                let mut depth = 1;
                while let Some(sub_next) = chars.next() {
                    match sub_next {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => {}
                    }
                    buf.push(sub_next as u8);
                    if depth == 0 {
                        break;
                    }
                }

                out.push(ParsedItem::Delayed(String::from_utf8(buf).unwrap()))
            }
            ']' => {
                if out.is_empty() {
                    out.push(ParsedItem::Empty);
                }
            }
            ',' => {}
            _ => {
                let mut num = next as u8 - b'0';

                let check_next = chars.next();
                if let Some(ch_next) = check_next {
                    if (ch_next as u8).is_ascii_digit() {
                        num = num * 10 + (ch_next as u8 - b'0')
                    }
                }

                out.push(ParsedItem::Num(num));
            }
        }
    }

    out.into_iter()
}
