pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();

    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            stack.push(ch);
        } else if ch == ')' {
            if !stack.is_empty() && stack.pop().unwrap() == '(' {
            } else {
                return false;
            }
        } else if ch == '}' {
            if !stack.is_empty() && stack.pop().unwrap() == '{' {
            } else {
                return false;
            }
        } else if ch == ']' {
            if !stack.is_empty() && stack.pop().unwrap() == '[' {
            } else {
                return false;
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_test() {
        let s = "()".to_string();
        assert_eq!(is_valid(s), true);
    }
}
