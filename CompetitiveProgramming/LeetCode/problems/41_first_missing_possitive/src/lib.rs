pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let mut i = 0;
    while i != nums.len() {
        if nums[i] == i as i32 + 1
            || nums[i] > nums.len() as i32
            || nums[i] < 1
            || (nums[i] <= nums.len() as i32 && nums[i] == nums[nums[i] as usize - 1])
        {
            i += 1;
        } else {
            let temp = nums[i];
            nums[i] = nums[temp as usize - 1];
            nums[temp as usize - 1] = temp;
        }
    }
    for i in 0..nums.len() {
        if i as i32 != nums[i] - 1 {
            return (i + 1) as i32;
        }
    }
    nums.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_missing_positive_test() {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![1, 1]), 2);
    }
}
