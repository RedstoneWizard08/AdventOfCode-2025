use std::fs;

// I KNOW there is a better way to do this, but I can't be bothered right now :P

fn dumb_move_left(mut orig: usize, amount: usize, clicks: &mut usize) -> usize {
    for _ in 0..amount {
        if orig == 0 {
            orig = 99;
        } else {
            orig -= 1;
        }

        if orig == 0 {
            *clicks += 1;
        }
    }

    orig
}

fn dumb_move_right(mut orig: usize, amount: usize, clicks: &mut usize) -> usize {
    for _ in 0..amount {
        if orig == 99 {
            orig = 0;
        } else {
            orig += 1;
        }

        if orig == 0 {
            *clicks += 1;
        }
    }

    orig
}

fn main() {
    let mut zeros = 0;
    let mut pos = 50;

    let text = fs::read_to_string("input.txt").unwrap();

    text.trim()
        .lines()
        .map(|it| {
            let mut chars = it.chars();
            let mode = chars.next().unwrap();
            let amount = chars.collect::<String>().parse::<usize>().unwrap();

            (mode, amount)
        })
        .for_each(|(mode, amount)| match mode {
            'L' => pos = dumb_move_left(pos, amount, &mut zeros),
            'R' => pos = dumb_move_right(pos, amount, &mut zeros),
            i => panic!("Unknown mode: {i}"),
        });

    println!("Zeros: {zeros}");
}
