#![feature(iter_next_chunk)]

use common::{read_lines, Timer};
use std::collections::{HashSet, VecDeque};

fn main() {
    let timer = Timer::start();

    let input = read_lines("input.txt").unwrap();
    let input_txt = input.last().unwrap().unwrap();

    println!(
        "number of chars 1 {}",
        find_marker_pos::<4>(input_txt.as_ref())
    );
    println!(
        "number of chars 2 {}",
        find_marker_pos::<14>(input_txt.as_ref())
    );

    timer.stop();
}

fn find_marker_pos<const N: usize>(input: &str) -> usize {
    let mut stream = input.chars();

    let mut buf = VecDeque::with_capacity(4);
    buf.extend(stream.next_chunk::<N>().unwrap().iter());

    let mut final_pos = N;
    for (pos, ch) in stream.enumerate() {
        let h: HashSet<char> = HashSet::from_iter(buf.iter().cloned());
        if h.len() == N {
            final_pos += pos;
            break;
        }

        buf.pop_front();
        buf.push_back(ch);
    }

    final_pos
}
