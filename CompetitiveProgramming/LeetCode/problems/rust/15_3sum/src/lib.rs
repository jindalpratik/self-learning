pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = std::collections::HashSet::new();
    for i in 0..(nums.len() - 2) {
        let mut j = i + 1;
        let mut k = nums.len() - 1;

        while j < k {
            let sum = nums[j] + nums[k] + nums[i];
            if sum == 0 {
                result.insert(vec![nums[i], nums[j], nums[k]]);
                while j < k && nums[j] == nums[j + 1] {
                    j += 1;
                }
                while j < k && nums[k] == nums[k - 1] {
                    k -= 1;
                }
                j += 1;
                k -= 1;
            } else if sum > 0 {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }

    result.into_iter().collect()
}
