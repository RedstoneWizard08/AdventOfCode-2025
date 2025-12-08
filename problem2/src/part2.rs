use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: &str = include_str!("../input.txt");

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
        .filter_map(|num| {
            // invalids:
            //     d = len / 2
            //     num = [n * 10^((d / chk) * 1)] + [n * 10^((d / chk) * 2)] + [n * 10^((d / chk) * 3)] + [...] + [n * 10^((d / chk) * chk)]
            //        ^^ probably not accurate, hard to put into an equation lol

            let d = num.ilog10() + 1;
            let d2 = d / 2;

            (1..=d2).find_map(|chk| {
                if d % chk != 0 {
                    return None;
                }

                let chunks = d / chk;
                let max = 10usize.pow(d - chk);
                let base = num / max;
                let mut target = base;

                for i in 1..chunks {
                    target += base * 10usize.pow(i * chk);
                }

                if num == target { Some(num) } else { None }
            })
        })
        .sum::<usize>();

    std::hint::black_box(sum);

    #[cfg(feature = "cli")]
    println!("Sum: {sum}");
}
