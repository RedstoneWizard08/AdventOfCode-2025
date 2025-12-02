#![feature(iter_order_by)]

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::fs;

fn main() {
    let sum = fs::read_to_string("input.txt")
        .unwrap()
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

            (1..(num_s.len() / 2 + 1)).find_map(|len| {
                let chunks = num_s.chars().chunks(len);
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
