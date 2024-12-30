pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for token in tokens {
        match token.as_str() {
            "+" => {
                let val_2 = stack.pop().unwrap();
                let val_1 = stack.pop().unwrap();
                stack.push(val_1 + val_2);
            }
            "-" => {
                let val_2 = stack.pop().unwrap();
                let val_1 = stack.pop().unwrap();
                stack.push(val_1 - val_2);
            }
            "*" => {
                let val_2 = stack.pop().unwrap();
                let val_1 = stack.pop().unwrap();
                stack.push(val_1 * val_2);
            }
            "/" => {
                let val_2 = stack.pop().unwrap();
                let val_1 = stack.pop().unwrap();
                stack.push(val_1 / val_2);
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    }

    if !stack.is_empty() {
        stack.pop().unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_rpn_test() {
        let tokens = ["2", "1", "+", "3", "*"]
            .into_iter()
            .map(|t| t.to_string())
            .collect();
        assert_eq!(eval_rpn(tokens), 9);
    }
}
