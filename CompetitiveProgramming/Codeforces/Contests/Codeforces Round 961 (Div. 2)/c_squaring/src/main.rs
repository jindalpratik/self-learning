use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let len = get_i32();

        let mut nums_arr = get_i32_vec();
        let mut count = 0;

        for i in 1..len as usize {
            if nums_arr[i] > nums_arr[i - 1] {
                continue;
            }

            if nums_arr[i] < nums_arr[i - 1] && nums_arr[i] == 1 {
                count = -1;
                break;
            }

            while nums_arr[i] < nums_arr[i - 1] {
                count += 1;
                nums_arr[i] = nums_arr[i] * nums_arr[i];
            }
        }

        println!("{}", count);
    }
}

fn get_i32() -> i32 {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.trim().parse().unwrap()
}

fn get_i32_vec() -> Vec<u256> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
