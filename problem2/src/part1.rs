use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("../input.txt");

#[cfg_attr(not(feature = "cli"), allow(unused))]
pub fn main() {
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
        .map(|num| {
            // invalid ID = n * 10^(num.len() / 2) + n
            let d = (num.ilog10() + 1) / 2;
            let base = num / 10_usize.pow(d);
            let target = (base * 10_usize.pow(d)) + base;

            if num == target { num } else { 0 }
        })
        .sum::<usize>();

    #[cfg(feature = "cli")]
    println!("Sum: {sum}");
}
