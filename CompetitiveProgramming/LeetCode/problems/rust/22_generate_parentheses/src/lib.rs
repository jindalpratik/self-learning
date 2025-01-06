pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut list = Vec::new();
    let mut str = String::new();
    let start = n as usize;
    let end = n as usize;
    recursive(&mut list, &mut str, start, end);
    list
}

pub fn recursive(list: &mut Vec<String>, str: &mut String, start: usize, end: usize) {
    if start == 0 && end == 0 {
        list.push(str.clone());
        return;
    }

    if start > 0 {  
        str.push('(');
        recursive(list, str, start - 1, end);
        str.pop();
    }

    if start < end {
        str.push(')');
        recursive(list, str, start, end - 1);
        str.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_parenthesis_test() {
        let n = 1;
        assert_eq!(generate_parenthesis(n), vec!["()".to_string()]);
    }
}
