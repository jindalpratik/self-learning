pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;
    loop {
        if numbers[i] + numbers[j] == target {
            // We can early return as there is exactly one solution.
            return vec![i as i32 + 1, j as i32 + 1];
        } else if numbers[i] + numbers[j] > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }
}
