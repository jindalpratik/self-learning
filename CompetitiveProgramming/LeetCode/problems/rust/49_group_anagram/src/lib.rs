use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for i in strs {
        let mut chars: Vec<_> = i.chars().collect();
        chars.sort_unstable();
        map.entry(chars).or_insert(vec![]).push(i);
    }

    map.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_anagrams_test() {
        let test_vec = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result_vec = vec![
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        ];
        // Test fails as the test is not order independent
        assert_eq!(group_anagrams(test_vec), result_vec);
    }
}
