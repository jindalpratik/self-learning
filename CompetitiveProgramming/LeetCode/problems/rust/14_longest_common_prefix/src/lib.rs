pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut index = 0;
    let mut result = "".to_string();
    loop {
        if index >= strs[0].len() {
            return result;
        }
        let current_char = strs[0].chars().nth(index).unwrap();
        for i in 1..strs.len() {
            if index >= strs[i].len() || current_char != strs[i].chars().nth(index).unwrap() {
                return result;
            }
        }
        result.push(current_char);
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        let strs = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strs), "fl".to_string());
    }
}
