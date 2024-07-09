use std::cmp::min;

pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len <= 4 {
        return 0;
    }
    nums.sort();
    let mut ans = i32::MAX;
    for k in 0..4 {
        ans = min(ans, nums[len - 4 + k] - nums[k]);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_difference_test() {
        assert_eq!(min_difference(vec![1, 5, 0, 10, 14]), 1);
    }
}
