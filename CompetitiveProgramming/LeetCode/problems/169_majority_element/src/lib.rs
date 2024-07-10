use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    let len = nums.len();

    for i in nums {
        let count = freq.entry(i).or_insert(0);
        *count += 1;
    }

    for (key, value) in freq {
        if value > len / 2 {
            return key;
        }
    }

    -1
}

pub fn majority_element_optimal(nums: Vec<i32>) -> i32 {
    let mut key = 0;
    let mut count = 0;

    for i in nums {
        if count == 0 {
            key = i;
        }
        if i == key {
            count += 1;
        } else {
            count -= 1;
        }
    }

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn majority_element_test() {
        assert_eq!(majority_element(vec![3,2,3]), 3)
    }

    #[test]
    fn majority_element_optimal_test() {
        assert_eq!(majority_element(vec![3,2,3]), 3)
    }
}
