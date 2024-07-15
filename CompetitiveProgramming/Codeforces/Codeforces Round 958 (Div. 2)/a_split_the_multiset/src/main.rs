use std::io::stdin;

fn main() {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    let test_cases: i32 = test_cases.trim().parse().unwrap();

    for _ in 0..test_cases {
        let mut nums = String::new();
        stdin().read_line(&mut nums).unwrap();
        let nums: Vec<i32> = nums
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        if nums[0] == 1 {
            println!("0");
        } else if nums[0] % nums[1] == 0 {
            println!("{}", (nums[0] / nums[1]) + 1);
        } else {
            println!("{}", (nums[0] / nums[1]) + 2);
        }
    }
}
