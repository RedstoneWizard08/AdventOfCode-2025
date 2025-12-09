use itertools::Itertools;
use macros::embed_positions;

embed_positions!("../input.txt");

pub fn main() {
    let max = INPUT
        .into_iter()
        .permutations(2)
        .map(|it| it.into_iter().collect_tuple::<(_, _)>().unwrap())
        .filter(|((x1, y1), (x2, y2))| *x1 < *x2 && *y1 < *y2)
        .map(|((x1, y1), (x2, y2))| (x2 - x1 + 1) * (y2 - y1 + 1))
        .max()
        .unwrap();

    println!("Max: {max}");
}
