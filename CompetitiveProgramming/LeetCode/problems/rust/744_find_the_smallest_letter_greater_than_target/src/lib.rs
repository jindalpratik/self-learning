pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    for i in 0..(letters.len() - 1) {
        if letters[i] <= target && letters[i + 1] > target {
            return letters[i + 1];
        }
    }
    letters[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_greatest_letter_test() {
        assert_eq!(next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c')
    }
}
