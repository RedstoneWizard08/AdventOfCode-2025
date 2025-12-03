use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn take_from(nums: &Vec<u32>, needed: usize, cursor: &mut usize) -> u32 {
    let avail = &nums[*cursor..=(nums.len() - 1 - needed)];
    let max = avail.iter().max().unwrap();
    let idx = avail.iter().position(|it| it == max).unwrap();

    *cursor += idx + 1;
    avail[idx]
}

#[cfg_attr(not(feature = "cli"), allow(unused))]
pub fn main() {
    let lines = INPUT.trim().lines().map(|it| it.to_owned()).collect_vec();
    let mut total = 0u64;

    for line in lines {
        let nums = line
            .chars()
            .map(|it| it.to_digit(10).unwrap())
            .collect_vec();

        let mut cursor = 0;

        let n0 = take_from(&nums, 1, &mut cursor);
        let n1 = take_from(&nums, 0, &mut cursor);

        let num = format!("{n0}{n1}").parse::<u64>().unwrap();

        total += num;
    }

    #[cfg(feature = "cli")]
    println!("Total: {total}");
}
