pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = nums.len() as i32 - 1;

    while start <= end {
        let i = (start + end) as usize / 2;
        if nums[i] == target {
            return i as i32;
        } else if nums[i] > target {
            end = i as i32 - 1;
        } else {
            start = i as i32 + 1;
        }
    }
    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_test() {
        assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
        assert_eq!(search_insert(vec![1,3,5,6], 3), 1);
        assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
    }
}
