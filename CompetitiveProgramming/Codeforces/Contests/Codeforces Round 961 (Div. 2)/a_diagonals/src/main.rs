use std::io::stdin;

fn main() {
    let test_cases = no_test_cases();

    for _ in 0..test_cases {
        let mut nums = String::new();
        stdin().read_line(&mut nums).unwrap();
        let nums: Vec<i32> = nums
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut size = nums[0];
        let mut step = nums[1];

        if step == 0 {
            println!("0");
            continue;
        }

        let mut count = 0;
        while step > 0 && size > 0{
            count += 1;
            step -= size;
            if count % 2 != 0 {
                size -= 1;
            }
        }

        println!("{}", count);
    }
}

fn no_test_cases() -> i32 {
    let mut test_cases = String::new();
    stdin().read_line(&mut test_cases).unwrap();
    test_cases.trim().parse().unwrap()
}
