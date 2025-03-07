use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let (length, side) = get_i32_tup();

        let mut sum_x = 0;
        let mut sum_y = 0;

        let (_init_x, _init_y) = get_i32_tup();
        for _ in 1..length {
            let (x, y) = get_i32_tup();
            sum_x += x;
            sum_y += y;
        }

        let sum = (sum_x + side) * 2 +  (sum_y + side) * 2;
        println!("{}", sum);
    }
}

fn get_i32() -> i32 {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.trim().parse().unwrap()
}

// fn get_i32_vec() -> Vec<i32> {
//     let mut nums = String::new();
//     stdin().read_line(&mut nums).unwrap();
//     nums.split_whitespace()
//         .map(|x| x.trim().parse().unwrap())
//         .collect()
// }

fn get_i32_tup() -> (i32, i32) {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    let nums: Vec<i32> = nums.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();
    (nums[0], nums[1])
}
