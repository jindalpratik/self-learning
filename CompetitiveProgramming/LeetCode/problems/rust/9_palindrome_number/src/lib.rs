pub fn is_palindrome(x: i32) -> bool {
    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

pub fn is_palindrome_2(x: i32) -> bool {
    x.to_string().chars().eq(x.to_string().chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn is_palindrome_2_test() {
        assert_eq!(is_palindrome_2(121), true);
    }
}
