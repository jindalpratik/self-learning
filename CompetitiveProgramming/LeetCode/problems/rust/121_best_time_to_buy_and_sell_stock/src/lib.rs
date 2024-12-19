pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_left = vec![99999;prices.len()];
    let mut min = 99999;
    for i in 0..prices.len() {
        min_left[i] = min;
        min = std::cmp::min(min, prices[i]);
    }
    let mut max = 0;
    for i in 0..prices.len() {
        max = std::cmp::max(max, prices[i] - min_left[i]);
    }
    max
}

pub fn max_profit_alt(prices: Vec<i32>) -> i32 {
    let mut buy = prices[0];
    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] < buy {
            buy = prices[i];
        }

        profit = std::cmp::max(profit, prices[i] - buy);
    }
    
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_test() {
        let prices = vec![7,1,5,3,6,4];
        assert_eq!(max_profit(prices.clone()), 5);
        assert_eq!(max_profit_alt(prices), 5);

    }
}
