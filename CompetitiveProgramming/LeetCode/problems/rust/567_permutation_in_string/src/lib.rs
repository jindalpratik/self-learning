pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut char_arr = [0; 26];
    for ch in s1.chars() {
        char_arr[ch as usize - 97] += 1;
    }

    let s2: Vec<_> = s2.chars().collect();
    let mut l = 0;
    let mut r = s1.len() - 1;
    let mut confirm_arr = [0; 26];

    for i in l..(r+1) {
        confirm_arr[s2[i] as usize - 97] += 1;
    }

    while r < s2.len() - 1 {
        if char_arr == confirm_arr {
            return true;
        }

        confirm_arr[s2[l] as usize - 97] -= 1;
        l += 1;
        r += 1;
        confirm_arr[s2[r] as usize - 97] += 1;
    }

    char_arr == confirm_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inclusion_test() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();
        assert_eq!(check_inclusion(s1, s2), true);
    }
}
