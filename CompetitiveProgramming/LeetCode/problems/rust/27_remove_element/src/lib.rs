pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while j < nums.len() {
        if nums[j] == val {
            j += 1;
        } else {
            nums[i] = nums[j];
            i += 1;
            j += 1;
        }
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_element_test() {
        let mut nums = vec![3,2,2,3];
        assert_eq!(remove_element(&mut nums, 3), 2);
        let mut nums = vec![0,1,2,2,3,0,4,2];
        assert_eq!(remove_element(&mut nums, 2), 5);
    }
}
