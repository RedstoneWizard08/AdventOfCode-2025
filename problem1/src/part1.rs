use std::fs;
use anyhow::Result;
use indicatif::ProgressIterator;
use itertools::Itertools;

// I KNOW there is a better way to do this, but I can't be bothered right now :P

fn dumb_move_left(mut orig: usize, amount: usize) -> usize {
    for _ in 0..amount {
        if orig == 0 {
            orig = 99;
        } else {
            orig -= 1;
        }
    }

    orig
}

fn dumb_move_right(mut orig: usize, amount: usize) -> usize {
    for _ in 0..amount {
        if orig == 99 {
            orig = 0;
        } else {
            orig += 1;
        }
    }

    orig
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?
        .trim()
        .lines()
        .map(|it| it.to_owned())
        .collect_vec();

    let mut zeros = 0;
    let mut pos = 50;

    for mut item in input.into_iter().progress() {
        let mode = item.remove(0);
        let amount = item.parse::<usize>()?;

        match mode {
            'L' => pos = dumb_move_left(pos, amount),
            'R' => pos = dumb_move_right(pos, amount),
            i => panic!("Unknown mode: {i}")
        }

        if pos == 0 {
            zeros += 1;
        }
    }

    println!("Zeros: {zeros}");

    Ok(())
}
