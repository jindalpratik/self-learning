use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let n_k = get_i32_vec();
        let _n = n_k[0];
        let k = n_k[1];

        let mut a = get_i32_vec();
        let b = get_i32_vec();

        let mut count: u64 = 0;
        for _i in 0..k {
            let mut index = 0;
            // dbg!(&a);
            for (j, &value) in a.iter().enumerate() {
                if value > a[index] {
                    index = j;
                }
            }
            if a[index] > 0 {
                count += a[index] as u64;
            } else {
                break;
            }
            a[index] = a[index] - b[index];
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
