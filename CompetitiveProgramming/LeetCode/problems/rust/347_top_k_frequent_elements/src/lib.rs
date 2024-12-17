pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    nums.into_iter()
        .for_each(|num| *map.entry(num).or_insert(0) += 1);
    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    vec.into_iter().take(k as usize).map(|x| x.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(top_k_frequent(nums, k), vec![1, 2])
    }
}
