use itertools::Itertools;
use macros::embed_lines;

embed_lines!("../input.txt");

pub fn main() {
    let mut present_sizes = [0; 6];
    let mut lines = INPUT.into_iter();

    for i in 0..=5 {
        lines.next().unwrap();

        present_sizes[i] = lines
            .next_array::<3>()
            .unwrap()
            .into_iter()
            .join("")
            .chars()
            .filter(|it| *it == '#')
            .count();

        lines.next().unwrap();
    }

    let count = lines
        .map(|line| {
            let w = &line[0..2];
            let h = &line[3..5];
            let spec = &line[7..];

            let w = w.parse::<usize>().unwrap();
            let h = h.parse::<usize>().unwrap();

            let size = w * h;

            let spec = spec
                .split(" ")
                .enumerate()
                .map(|(idx, it)| (idx, it.parse::<usize>().unwrap()))
                .map(|(idx, it)| present_sizes[idx] * it)
                .sum::<usize>();

            (spec <= size) as usize
        })
        .sum::<usize>();

    std::hint::black_box(count);

    #[cfg(feature = "cli")]
    println!("Count: {count}");
}
