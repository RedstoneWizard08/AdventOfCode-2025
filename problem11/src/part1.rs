use macros::embed_split_strip_colon_lines;
use std::collections::BTreeMap;

embed_split_strip_colon_lines!("../input.txt");

type Map = BTreeMap<&'static str, &'static [&'static str]>;

pub fn resolve_paths(map: &Map, mut visited: Vec<&'static str>, start: &'static str) -> usize {
    if start == "out" {
        1
    } else if visited.contains(&start) {
        0
    } else {
        visited.push(start);

        map[start]
            .into_iter()
            .map(|it| resolve_paths(map, visited.clone(), *it))
            .sum()
    }
}

pub fn main() {
    let mut map = Map::new();

    for (name, parts) in INPUT {
        map.insert(name, parts);
    }

    let res = resolve_paths(&map, Vec::new(), "you");

    std::hint::black_box(res);

    #[cfg(feature = "cli")]
    println!("Result: {res}");
}
