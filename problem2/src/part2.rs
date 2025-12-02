#![feature(iter_order_by)]

use itertools::Itertools;
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

            (1..=(num_s.len() / 2)).find_map(|chk| {
                if num_s.len() % chk != 0 {
                    return None;
                }

                let chunks = num_s.chars().chunks(chk);
                let mut bits = chunks.into_iter();
                let first = bits.next().unwrap().collect::<Vec<char>>();

                if bits.all(|it| it.eq_by(first.iter(), |x, y| x == *y)) {
                    Some(num)
                } else {
                    None
                }
            })
        })
        .sum::<usize>();

    println!("Sum: {sum}");
}
