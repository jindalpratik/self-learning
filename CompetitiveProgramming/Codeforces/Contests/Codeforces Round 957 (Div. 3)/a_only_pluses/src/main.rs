use std::io::stdin;

fn main() {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _ in 0..test_cases {
        let mut nums = String::new();
        stdin().read_line(&mut nums).unwrap();
        let mut nums: Vec<i32> = nums
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        nums.sort();

        for _ in 0..5 {
            if nums[0] < nums[1] {
                nums[0] += 1;
            } else if nums[1] < nums[2] {
                nums[1] += 1;
            } else {
                nums[2] += 1;
            }
        }

        let mut mult = 1;
        for i in nums {
            mult *= i;
        }
        println!("{}", mult);
    }
}
