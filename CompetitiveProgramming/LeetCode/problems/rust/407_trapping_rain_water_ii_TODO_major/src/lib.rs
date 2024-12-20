pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    let m = height_map.len();
    let n = height_map[0].len();

    if m < 3 || n < 3 {
        return 0;
    }

    let mut left_max = vec![vec![0; n]; m];
    let mut right_max = vec![vec![0; n]; m];
    let mut top_max = vec![vec![0; n]; m];
    let mut bottom_max = vec![vec![0; n]; m];

    for i in 0..m {
        let mut max = 0;
        for j in 0..n {
            left_max[i][j] = max;
            max = std::cmp::max(max, height_map[i][j]);
        }
    }

    for i in 0..n {
        let mut max = 0;
        for j in 0..m {
            top_max[j][i] = max;
            max = std::cmp::max(max, height_map[j][i]);
        }
    }

    for i in 0..m {
        let mut max = 0;
        for j in (0..n).rev() {
            right_max[i][j] = max;
            max = std::cmp::max(max, height_map[i][j]);
        }
    }

    for i in 0..n {
        let mut max = 0;
        for j in (0..m).rev() {
            bottom_max[j][i] = max;
            max = std::cmp::max(max, height_map[j][i]);
        }
    }

    let mut water = 0;
    for i in 0..m {
        for j in 0..n {
            let arr = vec![
                left_max[i][j],
                right_max[i][j],
                top_max[i][j],
                bottom_max[i][j],
            ];
            let minimum = arr.iter().min().unwrap();
            if minimum <= &height_map[i][j] {
                continue;
            }
            water += minimum - height_map[i][j];
        }
    }

    water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_rain_water_test() {
        let height_map = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        assert_eq!(trap_rain_water(height_map), 4);
    }
}
