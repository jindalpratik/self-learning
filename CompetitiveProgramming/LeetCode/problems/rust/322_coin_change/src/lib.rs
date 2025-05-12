pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coin_change_test() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(coin_change(vec![2], 3), -1);
        assert_eq!(coin_change(vec![1], 0), 0);
        assert_eq!(coin_change(vec![1], 2), 2);
    }
}
