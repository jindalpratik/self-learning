pub fn character_replacement(s: String, k: i32) -> i32 {
    let chars: Vec<_> = s.chars().collect();
    let mut char_arr = vec![0; 26];
    let mut max_repeat = 0;
    let mut max_len = 0;
    let mut l = 0;
    for i in 0..chars.len() {
        let curr = chars[i];
        char_arr[curr as usize - 65] += 1;
        max_len = max_len.max(char_arr[curr as usize - 65]);
        while i as i32 - l as i32 - max_len >= k {
            char_arr[chars[l] as usize - 65] -= 1;
            l += 1;
        }
        max_repeat = max_repeat.max(i - l + 1);
    }
    max_repeat as i32
}

pub fn character_replacement_alt(s: String, k: i32) -> i32 {
    let chars: Vec<_> = s.chars().collect();
    let mut char_arr = vec![0; 26];
    let mut max_repeat = 0;
    let mut l = 0;
    for i in 0..chars.len() {
        let curr = chars[i];
        char_arr[curr as usize - 65] += 1;
        while i as i32 - l as i32 - char_arr.iter().max().unwrap() >= k {
            char_arr[chars[l] as usize - 65] -= 1;
            l += 1;
        }
        max_repeat = max_repeat.max(i - l + 1);
    }
    max_repeat as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_replacement_test() {
        let s = "XYYX".to_string();
        assert_eq!(character_replacement(s, 2), 4);
        let s = "AABABBA".to_string();
        assert_eq!(character_replacement(s, 1), 4);
    }
}
