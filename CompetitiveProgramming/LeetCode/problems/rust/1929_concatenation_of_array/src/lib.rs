pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.repeat(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_concatenation_test() {
        let nums = vec![1,2,1];
        let res = vec![1,2,1,1,2,1];
        assert_eq!(get_concatenation(nums), res);
    }
}
