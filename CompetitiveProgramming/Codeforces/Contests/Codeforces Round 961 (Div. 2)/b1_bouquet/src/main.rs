use std::io::stdin;

fn main() {
    let test_cases = no_test_cases();

    for _ in 0..test_cases {
        let nums = get_i32_vec();

        let no_flowers = nums[0];
        let coins = nums[1];

        let mut nums = get_i32_vec();
        nums.sort();

        let mut max_petals = 0;
        let mut current_petals = 0;
        let mut current_cost = 0;

        for i in 0..no_flowers as usize {
            if i > 0 && nums[i] - nums[i - 1] > 1 {
                max_petals = std::cmp::max(max_petals, current_petals);
                current_petals = 0;
                current_cost = 0;
            }
            current_cost += nums[i];
            if current_cost <= coins {
                current_petals += nums[i];
                max_petals = std::cmp::max(max_petals, current_petals);
            } else {
                break;
            }
        }

        println!("{}", max_petals);
    }
}

fn no_test_cases() -> i32 {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    test_cases.trim().parse().unwrap()
}

fn get_i32_vec() -> Vec<i32> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
