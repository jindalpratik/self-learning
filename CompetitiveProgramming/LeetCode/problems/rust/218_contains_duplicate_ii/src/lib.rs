use std::collections::HashSet;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut set = HashSet::new();

    let count = if k < nums.len() as i32 {
        k
    } else {
        nums.len() as i32
    };
    for i in 0..count {
        if set.contains(&nums[i as usize]) {
            return true;
        }
        set.insert(nums[i as usize]);
    }

    for i in (k as usize)..nums.len() {
        if set.contains(&nums[i]) {
            return true;
        }
        set.insert(nums[i as usize]);
        set.remove(&nums[i - k as usize]);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_duplicate_test() {
        let vec = vec![1, 2, 3, 1];
        assert_eq!(contains_nearby_duplicate(vec, 3), true);
        assert_eq!(contains_nearby_duplicate(vec![-1, -1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true);
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false);
        assert_eq!(
            contains_nearby_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 15),
            false
        );
    }
}
