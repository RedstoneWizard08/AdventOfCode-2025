use std::fs;

fn move_left(orig: usize, amount: usize) -> usize {
    let temp = (orig as isize - amount as isize) % 100;

    if temp < 0 {
        (100 + temp) as usize
    } else {
        temp as usize
    }
}

fn move_right(orig: usize, amount: usize) -> usize {
    (orig + amount) % 100
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
        .for_each(|(mode, amount)| {
            match mode {
                'L' => pos = move_left(pos, amount),
                'R' => pos = move_right(pos, amount),
                i => panic!("Unknown mode: {i}"),
            }

            if pos == 0 {
                zeros += 1;
            }
        });

    println!("Zeros: {zeros}");
}
