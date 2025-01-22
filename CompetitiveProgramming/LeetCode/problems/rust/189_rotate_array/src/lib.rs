pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let mut new_vec = Vec::new();
    let n = nums.len();
    let k = k as usize % n;
    for i in n - k..n {
        new_vec.push(nums[i]);
    }
    for i in 0..n - k {
        new_vec.push(nums[i]);
    }
    for i in 0..n {
        nums[i] = new_vec[i];
    }
}
