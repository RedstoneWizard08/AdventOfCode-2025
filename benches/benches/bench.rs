macro_rules! bench_modules {
    [
        $($problem: ident = $name: expr),*
        $(,)? // trailing comma lol
    ] => {
        $(
            bench_modules!($problem, $name);
        )*

        criterion::criterion_main!($($problem),*);
    };

    ($problem: ident, $name: expr) => {
        mod $problem {
            use criterion::{Criterion, criterion_group};

            fn bench_part_1(c: &mut Criterion) {
                c.bench_function(concat!($name, " - part 1"), |b| b.iter(|| $problem::part1::main()));
            }

            fn bench_part_2(c: &mut Criterion) {
                c.bench_function(concat!($name, " - part 2"), |b| b.iter(|| $problem::part2::main()));
            }

            criterion_group!($problem, bench_part_1, bench_part_2);
        }

        use $problem::$problem as $problem;
    };
}

bench_modules![
    problem1 = "problem 1",
    problem2 = "problem 2",
    problem3 = "problem 3",
    problem4 = "problem 4",
    problem5 = "problem 5",
    problem6 = "problem 6",
    // problem7 = "problem 7",
    // problem8 = "problem 8",
    // problem9 = "problem 9",
    // problem10 = "problem 10",
    // problem11 = "problem 11",
    // problem12 = "problem 12",
];
