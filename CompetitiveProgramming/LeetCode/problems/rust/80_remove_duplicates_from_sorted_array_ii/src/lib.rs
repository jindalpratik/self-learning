pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut count = 1;
    for j in 1..nums.len() {
        if nums[i] == nums[j] {
            count += 1;
            if count < 3 {
                i += 1;
                nums[i] = nums[j]
            }
        } else {
            i += 1;
            nums[i] = nums[j];
            count = 1;
        }
    }
    i as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_test() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates(&mut nums), 5);
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(remove_duplicates(&mut nums), 7);
    }
}
