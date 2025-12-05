use itertools::Itertools;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../input.txt");

#[cfg_attr(not(feature = "cli"), allow(unused))]
pub fn main() {
    let (pre, post) = INPUT.trim().split_once("\n\n").unwrap();
    let ranges = pre.trim().lines().map(|it| it.to_owned()).collect_vec();

    let ids = post
        .trim()
        .lines()
        .map(|it| it.parse::<u64>().unwrap())
        .collect_vec();

    let ranges = ranges
        .into_iter()
        .map(|it| {
            let (s, e) = it.split_once("-").unwrap();

            (s.parse::<u64>().unwrap(), e.parse::<u64>().unwrap())
        })
        .sorted_unstable_by_key(|it| it.0)
        .collect_vec();

    let mut all = Vec::<RangeInclusive<u64>>::new();

    for (s, e) in ranges {
        let range = s..=e;
        let existing = all.last_mut();

        if let Some(existing) = existing
            && *existing.end() >= s
        {
            *existing = *existing.start()..=(*existing.end()).max(e);
        } else {
            all.push(range);
        }
    }

    let ok = ids
        .into_iter()
        .filter(|it| all.iter().any(|r| r.contains(it)))
        .count();

    #[cfg(feature = "cli")]
    println!("Fresh: {ok}");
}
