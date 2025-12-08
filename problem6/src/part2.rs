use common::MultiCursor;
use macros::embed_input;

embed_input!("../input.txt");

pub type NUM = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProblemKind {
    Add,
    Mul,
    None,
}

pub fn main() {
    let mut cursor = MultiCursor::new(INPUT);
    let mut problems = Vec::new();

    let mut nums = Vec::new();
    let mut current_kind = ProblemKind::None;

    while let Some(next) = cursor.next() {
        if next.iter().all(|it| it.is_whitespace()) {
            problems.push((nums, current_kind));

            nums = Vec::new();
            current_kind = ProblemKind::None;

            continue;
        }

        let last = next[HEIGHT - 1];

        match last {
            '+' => current_kind = ProblemKind::Add,
            '*' => current_kind = ProblemKind::Mul,
            _ => {}
        }

        let mut num = 0;

        for i in 0..HEIGHT - 1 {
            let ch = next[i];

            if ch.is_digit(10) {
                num *= 10;
                num += ch.to_digit(10).unwrap() as NUM;
            }
        }

        nums.push(num);
    }

    problems.push((nums, current_kind));

    let res = problems
        .into_iter()
        .map(|(nums, kind)| match kind {
            ProblemKind::Add => nums.into_iter().sum::<NUM>(),
            ProblemKind::Mul => nums.into_iter().product::<NUM>(),
            ProblemKind::None => panic!("Problem kind should not be none!"),
        })
        .sum::<NUM>();

    std::hint::black_box(res);

    #[cfg(feature = "cli")]
    println!("Result: {res}");
}
