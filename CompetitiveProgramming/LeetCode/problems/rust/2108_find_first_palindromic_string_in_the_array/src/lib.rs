pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words {
        if word == word.chars().rev().collect::<String>() {
            return word;
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_palindrome_test() {
        assert_eq!(first_palindrome(vec!["abc".to_string(),"car".to_string(),"ada".to_string(),"racecar".to_string(),"cool".to_string()]),  "ada".to_string())
    }
}
