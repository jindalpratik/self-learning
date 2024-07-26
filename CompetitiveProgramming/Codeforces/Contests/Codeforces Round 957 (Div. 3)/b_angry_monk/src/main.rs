use std::io::stdin;

fn main() {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _ in 0..test_cases {
        let mut _dimens_nums = String::new();
        stdin().read_line(&mut _dimens_nums).unwrap();

        let mut nums = String::new();
        stdin().read_line(&mut nums).unwrap();
        let mut nums: Vec<i32> = nums
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        nums.sort();

        let mut count = 0;
        for i in 0..(nums.len() - 1) {
            count += nums[i] + nums[i] - 1;
        }
        println!("{}", count);
    }
}