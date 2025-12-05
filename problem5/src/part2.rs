use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

// DOES NOT WORK
pub fn main() {
    let (pre, _) = INPUT.trim().split_once("\n\n").unwrap();
    let ranges = pre.trim().lines().map(|it| it.to_owned()).collect_vec();

    let mut min = 0u64;
    let mut max = 0u64;

    for item in ranges {
        let (start, end) = item.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        min = min.min(start);
        max = max.max(end);
    }

    println!("Fresh: {}", max - min);
}
