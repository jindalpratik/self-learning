pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set = std::collections::HashSet::new();

    let mut l = 0;
    let mut max_len = 0;

    for (count, r) in s.chars().enumerate() {
        while set.contains(&r) {
            set.remove(&s.chars().nth(l).unwrap()); 
            l += 1;
        }

        set.insert(r);
        max_len = std::cmp::max(max_len, count - l + 1);
    }

    max_len as i32
}

pub fn length_of_longest_substring_optimised(s: String) -> i32 {
    let mut max_len = 0;
    let mut ascii_array = [0;128];
    let mut start = 0;

    for (end, ch) in s.chars().enumerate() {
        start = start.max(ascii_array[ch as usize]);
        max_len = max_len.max(end - start + 1);
        ascii_array[ch as usize] = end + 1;
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring_test() {
        let s = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring(s), 3);
        let s = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring(s), 1);
        let s = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring_optimised(s), 1);
    }
}
