const INPUT: &str = include_str!("../input.txt");

// I KNOW there is a better way to do this, but I can't be bothered right now :P

fn dumb_move(left: bool, mut orig: isize, amount: isize, clicks: &mut usize) -> isize {
    let step = if left { -1 } else { 1 };

    for _ in 0..amount {
        if orig == 0 && left {
            orig = 99;
        } else if orig == 99 && !left {
            orig = 0;
        } else {
            orig += step;
        }

        if orig == 0 {
            *clicks += 1;
        }
    }

    orig
}

#[cfg_attr(not(feature = "cli"), allow(unused_mut))]
pub fn main() {
    let mut zeros = 0;
    let mut pos = 50;

    INPUT
        .trim()
        .lines()
        .map(|it| {
            let mut chars = it.chars();
            let mode = chars.next().unwrap();
            let amount = chars.collect::<String>().parse::<isize>().unwrap();

            (mode, amount)
        })
        .for_each(|(mode, amount)| match mode {
            'L' => pos = dumb_move(true, pos, amount, &mut zeros),
            'R' => pos = dumb_move(false, pos, amount, &mut zeros),
            i => panic!("Unknown mode: {i}"),
        });

    #[cfg(feature = "cli")]
    println!("Zeros: {zeros}");
}
