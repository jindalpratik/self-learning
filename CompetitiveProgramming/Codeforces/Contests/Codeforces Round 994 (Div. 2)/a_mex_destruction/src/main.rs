use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let _length = get_i32();
        let vector = get_i32_vec();
        let mut count = 0;
        for i in 0..vector.len() {
            if vector[i] != 0 && count == 0{
                count = 1;
            } else if count == 1 && i < vector.len() - 1 && vector[i] == 0 && vector[i + 1] > 0 {
                count = 2;
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

fn get_i32_vec() -> Vec<i32> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
