use std::io::stdin;

fn main() {
    let test_cases = get_i32();

    for _ in 0..test_cases {
        let _length = get_i32();
        let mut string = String::new();
        stdin().read_line(&mut string).unwrap();
        let string = string.trim();

        let mut late_s = false;
        let mut early_s = false;    
        let mut seen_p = false;
        let mut flag = true;
        for (index, char) in string.chars().enumerate() {
            if index == 0 && char == 's' {
                early_s = true;
            } else if char == 's' {
                if seen_p {
                    flag = false;
                    break;
                } else {
                    late_s = true;
                }
            } else if char == 'p' {
                if early_s {
                    seen_p = true;
                } else if late_s && index == string.len() - 1 {
                    break;
                } else if late_s {
                    flag = false;
                    break;
                }
                else {
                    seen_p = true;
                }
            }
        }
        if flag {
            println!("YES");
        } else {
            println!("NO");
        }
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
