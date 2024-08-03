pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut operations_required = 0;
    for i in logs {
        operations_required = match i.as_str() {
            "../" => {
                if operations_required > 0 {
                    operations_required - 1
                } else {
                    operations_required
                }
            }
            "./" => operations_required,
            _ => operations_required + 1,
        }
    }
    operations_required
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_operations_test() {
        assert_eq!(min_operations(vec!["d1/".to_string(),"d2/".to_string(),"../".to_string(),"d21/".to_string(),"./".to_string()]), 2)
    }
}
