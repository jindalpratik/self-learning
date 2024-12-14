pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();

    for ch in s.chars() {
        map.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    for ch in t.chars() {
        map.entry(ch).and_modify(|counter| *counter -= 1).or_insert(-1);
    }

    map.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_test() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        assert_eq!(is_anagram(s, t), true);
    }

}
