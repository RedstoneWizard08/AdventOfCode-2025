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
        
        if num_s.len() % 2 != 0 {
            continue;
        }

        let len = num_s.len() / 2;
        let first = &num_s[0..len];
        let last = &num_s[len..];

        if first == last {
            sum += num;
        }
    }

    println!("Sum: {sum}");

    Ok(())
}
