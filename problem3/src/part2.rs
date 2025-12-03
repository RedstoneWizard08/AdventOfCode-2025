use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn take_from(nums: &Vec<u32>, needed: usize, cursor: &mut usize) -> u32 {
    let avail = &nums[*cursor..=(nums.len() - 1 - needed)];
    let max = avail.iter().max().unwrap();
    let idx = avail.iter().position(|it| it == max).unwrap();

    *cursor += idx + 1;
    avail[idx]
}

fn main() {
    let lines = INPUT.trim().lines().map(|it| it.to_owned()).collect_vec();
    let mut total = 0u64;

    for line in lines {
        let nums = line
            .chars()
            .map(|it| it.to_digit(10).unwrap())
            .collect_vec();

        let mut cursor = 0;

        let n0 = take_from(&nums, 11, &mut cursor);
        let n1 = take_from(&nums, 10, &mut cursor);
        let n2 = take_from(&nums, 9, &mut cursor);
        let n3 = take_from(&nums, 8, &mut cursor);
        let n4 = take_from(&nums, 7, &mut cursor);
        let n5 = take_from(&nums, 6, &mut cursor);
        let n6 = take_from(&nums, 5, &mut cursor);
        let n7 = take_from(&nums, 4, &mut cursor);
        let n8 = take_from(&nums, 3, &mut cursor);
        let n9 = take_from(&nums, 2, &mut cursor);
        let n10 = take_from(&nums, 1, &mut cursor);
        let n11 = take_from(&nums, 0, &mut cursor);

        let num = format!("{n0}{n1}{n2}{n3}{n4}{n5}{n6}{n7}{n8}{n9}{n10}{n11}").parse::<u64>().unwrap();

        total += num;
    }

    println!("Total: {total}");
}
