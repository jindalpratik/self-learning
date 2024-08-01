pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut count = 0;
    for passenger in details {
        if &passenger[11..13] > "60" {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_seniors_test() {
        assert_eq!(count_seniors(vec!["7868190130M7522".to_string(),"5303914400F9211".to_string(),"9273338290F4010".to_string()]), 1)
    }
}
