use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let legs = get_i32();

        let mut number = legs/4;
        if legs % 4 > 0 {
            number += 1;
        }

        println!("{}", number);
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
