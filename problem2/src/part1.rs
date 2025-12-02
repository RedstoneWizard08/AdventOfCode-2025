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

            if num_s.len() % 2 != 0 {
                return None;
            }

            let len = num_s.len() / 2;
            let first = &num_s[0..len];
            let last = &num_s[len..];

            if first == last { Some(num) } else { None }
        })
        .sum::<usize>();

    println!("Sum: {sum}");
}
