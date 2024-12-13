use std::collections::HashMap;

pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut height_name_map = HashMap::new();
    for i in 0..heights.len() {
        height_name_map.insert(heights[i], &names[i]);
    }

    let mut vec = Vec::new();
    let mut sorted_heights: Vec<_> = height_name_map.keys().cloned().collect();
    sorted_heights.sort_by(|a, b| b.cmp(a));
    for height in sorted_heights {
        vec.push(height_name_map[&height].clone());
    }
    
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_people_test() {
        let test_names = vec!["Mary".to_string(),"John".to_string(),"Emma".to_string()];
        let test_heights = vec![180,165,170];
        assert_eq!(sort_people(test_names, test_heights), vec!["Mary".to_string(),"Emma".to_string(),"John".to_string()]);
    }
}
