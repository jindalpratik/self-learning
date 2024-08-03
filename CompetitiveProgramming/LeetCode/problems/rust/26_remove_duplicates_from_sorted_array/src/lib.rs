pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 1;
    while j < nums.len() {
        if nums[i] == nums[j] {
            j += 1;
        } else {
            i += 1;
            nums[i] = nums[j];
            j += 1;
        }
    }
    i as i32 + 1
}

pub fn remove_duplicates_rust_methods(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates_test() {
        let mut nums = vec![1,1,2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }
}
