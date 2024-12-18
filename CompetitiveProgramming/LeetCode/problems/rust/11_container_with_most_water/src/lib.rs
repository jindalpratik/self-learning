pub fn max_area(height: Vec<i32>) -> i32 {
    let mut maximum_area = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    while i < j {
        let curr_area = (j as i32 - i as i32) * std::cmp::min(height[i], height[j]);
        maximum_area = std::cmp::max(maximum_area, curr_area);
        if height[i] <= height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    maximum_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area_test() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }
}
