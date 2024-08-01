use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let n_k = get_i32_vec();

        let k = n_k[1];
        let a = get_char_vec();
        let b = get_char_vec();
        dbg!(&a,&b);

        for _i in 0..k {
            let m_t = get_usize_vec();
            let m = m_t[0] -1;
            let t = m_t[1] -1;
            let mut char_vec = vec![0; 26];
            for it in a[m..t].iter().zip(b[m..t].iter()) {
                let (ai, bi) = it;
                char_vec[ai.to_ascii_lowercase() as usize - 97] += 1;
                char_vec[bi.to_ascii_lowercase() as usize - 97] -= 1;
            }

            let mut count = 0;
            for i in char_vec {
                if i < 0 {
                    count += 1;
                }
            }
            println!("{}", count)
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
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn get_usize_vec() -> Vec<usize> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums.split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn get_char_vec() -> Vec<char> {
    let mut nums = String::new();
    stdin().read_line(&mut nums).unwrap();
    nums = nums.trim().to_string();
    nums.chars().collect()
}
