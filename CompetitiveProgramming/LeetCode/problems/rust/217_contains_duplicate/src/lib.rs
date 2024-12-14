use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for i in nums {
        if set.contains(&i) {
            return true;
        }
        set.insert(i);
    }
    false
}

pub fn contains_duplicate_alternative(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for i in &nums {
        set.insert(i);
    }
    nums.len() != set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_test() {
        let vec = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(vec), true);
    }
}
