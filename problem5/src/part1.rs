use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

pub fn main() {
    let (pre, post) = INPUT.trim().split_once("\n\n").unwrap();
    let ranges = pre.trim().lines().map(|it| it.to_owned()).collect_vec();
    let ids = post.trim().lines().map(|it| it.to_owned()).collect_vec();

    let mut fresh = Vec::new();

    for item in ranges {
        let (start, end) = item.split_once("-").unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        fresh.push(start..=end);
    }

    let mut ok = 0;

    for id in ids {
        let id = id.parse::<u64>().unwrap();

        if fresh.iter().any(|it| it.contains(&id)) {
            ok += 1;
        }
    }

    println!("Fresh: {ok}");
}
