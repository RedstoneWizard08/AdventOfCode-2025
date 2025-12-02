use std::fs;

use anyhow::Result;
use indicatif::ProgressIterator;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?
        .trim()
        .split(",")
        .map(|it| it.to_owned())
        .collect_vec();

    let input = input
        .into_iter()
        .progress()
        .map(|it| {
            let (first, last) = it.split_once("-").unwrap();
            let first = first.parse::<usize>().unwrap();
            let last = last.parse::<usize>().unwrap();

            first..last
        })
        .flatten()
        .collect_vec();

    let mut sum = 0;

    for num in input.into_iter().progress() {
        let num_s = format!("{num}");
        let len = num_s.len() / 2;

        for len in 1..(len + 1) {
            let mut bits = num_s
                .chars()
                .chunks(len)
                .into_iter()
                .map(|it| it.collect::<String>())
                .collect_vec();

            let first = bits.remove(0);

            if bits.into_iter().all(|it| *it == first) {
                sum += num;
                break;
            }
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
