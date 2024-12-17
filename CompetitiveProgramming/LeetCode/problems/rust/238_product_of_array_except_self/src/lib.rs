pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut prod_r = 1;
    let mut prod_l = 1;
    for i in 0..nums.len() {
        ans.push(prod_r);
        prod_r = prod_r * nums[i];
    }
    for i in (0..nums.len()).rev() {
        ans[i] *= prod_l;
        prod_l *= nums[i];
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_except_self_test() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
