pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 3 {
        return 0;
    }
    let mut prefix_max = vec![0; height.len()];
    let mut suffix_max = vec![0; height.len()];
    let mut max = 0;
    for i in 0..height.len() {
        prefix_max[i] = max;
        max = std::cmp::max(max, height[i]);
    }
    max = 0;
    for i in (0..height.len()).rev() {
        suffix_max[i] = max;
        max = std::cmp::max(max, height[i]);
    }

    let mut water = 0;
    for i in 0..height.len() {
        let minimum = std::cmp::min(prefix_max[i], suffix_max[i]);
        if minimum < height[i] {
            continue;
        } else {
            water += minimum - height[i];
        }
    }

    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_test() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }
}
