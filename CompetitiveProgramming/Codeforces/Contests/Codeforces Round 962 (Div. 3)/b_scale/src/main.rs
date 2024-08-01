use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let n_k = get_i32_vec();
        let n = n_k[0];
        let k = n_k[1];
        let mut grid: Vec<Vec<u32>> = Vec::new();

        for _i in 0..n {
            let temp = get_i32_vec_2();
            grid.push(temp);
        }

        for i in 0..(n/k) {
            for f  in 0..(n/k) {
                print!("{}", grid[(i*k) as usize][(f*k) as usize]);
            }
            println!();
        }
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
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_i32_vec_2() -> Vec<u32> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums = nums.trim().to_string();
    nums.chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}