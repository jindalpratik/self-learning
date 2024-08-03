use std::collections::HashMap;

fn main() {
    println!("{:?}",two_sum(vec![2,7,11,15], 9));
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let mut hm = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        match hm.get(&num) {
            Some(&j) => return vec![j as i32, i as i32],
            None => {
                hm.insert(target - num, i);
            }
        }
    }

    unreachable!();
}
