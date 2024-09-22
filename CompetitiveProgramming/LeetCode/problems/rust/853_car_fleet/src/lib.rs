pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    cars.sort();
    let mut time_taken = Vec::new();

    for i in cars {
        let k = (target as f32 - i.0 as f32) / i.1 as f32;
        time_taken.push(k);
    }

    let mut count = 1;
    time_taken.reverse();

    let mut largest = time_taken[0];
    for i in 1..time_taken.len() {
        // println!("{}",time_taken[i]);
        if largest < time_taken[i] {
            largest = time_taken[i];
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_fleet_test() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
        assert_eq!(car_fleet(10, vec![3], vec![3]), 1);
        assert_eq!(car_fleet(10, vec![0, 4, 2], vec![2, 1, 3]), 1);
        assert_eq!(
            car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]),
            6
        );
    }
}
