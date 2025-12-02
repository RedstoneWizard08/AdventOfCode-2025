#![feature(iter_order_by)]

use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sum = INPUT
        .trim()
        .split(",")
        .par_bridge()
        .flat_map(|it| {
            let (first, last) = it.split_once("-").unwrap();
            let first = first.parse::<usize>().unwrap();
            let last = last.parse::<usize>().unwrap();

            first..last
        })
        .filter_map(|num| {
            let num_s = num.to_string();
            let first = num_s.chars().next().unwrap();

            if num_s.chars().filter(|it| *it == first).count() < 2 {
                return None;
            }

            let s_len = num_s.len();

            (1..=(s_len / 2)).find_map(|chk| {
                if s_len % chk != 0 {
                    return None;
                }

                let first = &num_s[..chk];
                let repeats = s_len / chk;

                if num_s == first.repeat(repeats) {
                    Some(num)
                } else {
                    None
                }
            })
        })
        .sum::<usize>();

    println!("Sum: {sum}");
}
