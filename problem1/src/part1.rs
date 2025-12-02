const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut zeros = 0;
    let mut pos = 50;

    INPUT.trim()
        .lines()
        .map(|it| {
            let mut chars = it.chars();
            let mode = chars.next().unwrap();
            let amount = chars.collect::<String>().parse::<isize>().unwrap();

            (mode, amount)
        })
        .for_each(|(mode, amount)| {
            match mode {
                'L' => pos = (pos - amount) % 100,
                'R' => pos = (pos + amount) % 100,
                i => panic!("Unknown mode: {i}"),
            }

            if pos == 0 {
                zeros += 1;
            }
        });

    println!("Zeros: {zeros}");
}
