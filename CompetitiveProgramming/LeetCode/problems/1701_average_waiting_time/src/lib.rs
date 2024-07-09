pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut cur_time = 0;
    let mut waiting_time = 0.0;
    let len = customers.len() as f64;
    for i in &customers {
        if cur_time < i[0] {
            cur_time = i[0];
        }
        cur_time += i[1];
        waiting_time += (cur_time - i[0]) as f64 / len;
    }
    waiting_time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_waiting_time_test() {
        assert_eq!(
            average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]),
            3.25000
        )
    }
}
