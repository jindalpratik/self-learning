pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut set = std::collections::HashSet::new();

    for i in nums {
        set.insert(i);
    }
    let mut max_consecutive = 0;
    for i in &set {
        if set.contains(&(i - 1)) {
            continue;
        }

        let mut consecutive = 1;
        let mut current = *i;
        while set.contains(&(current + 1)) {
            current += 1;
            consecutive += 1;
        }

        max_consecutive = std::cmp::max(max_consecutive, consecutive);
    }

    max_consecutive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_consecutive_test() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);

        let nums = vec![0, 0];
        assert_eq!(longest_consecutive(nums), 1);

        let nums = vec![];
        assert_eq!(longest_consecutive(nums), 0);
    }
}
