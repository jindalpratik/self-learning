use std::collections::BTreeMap;

pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    let mut new_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    for i in nums {
        let mut temp = i;
        let mut new = 0;
        let mut mult = 1;
        if temp == 0 {
            let x = new_map.entry(mapping[0]).or_insert(Vec::new());
            x.push(i);
            continue;
        }
        while temp != 0 {
            new += mapping[(temp % 10) as usize] * mult;
            mult *= 10;
            temp /= 10;
        }
        let x = new_map.entry(new).or_insert(Vec::new());
        x.push(i);
    }

    new_map.values().flatten().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_jumbled_test1() {
        let result = sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]);
        assert_eq!(result, vec![338, 38, 991]);
    }

    #[test]
    fn sort_jumbled_test2() {
        let result = sort_jumbled(
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        );
        assert_eq!(result, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }
}
