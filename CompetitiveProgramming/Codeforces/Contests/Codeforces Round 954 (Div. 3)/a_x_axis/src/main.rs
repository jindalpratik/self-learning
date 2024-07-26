use std::io::{self};

fn main() {
    let mut test_cases = String::new();

    io::stdin().read_line(&mut test_cases).unwrap();

    for _ in 0..test_cases.trim().parse().unwrap() {
        let mut nums = String::new();

        io::stdin().read_line(&mut nums).unwrap();

        let mut nums: Vec<i32> = nums
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        nums.sort();

        let median = nums[1];
        let mut distance = 0;
        for i in nums {
            distance += (i - median).abs();
        }

        println!("{distance}");
    }
}
