pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut vec = Vec::new();
    for i in 1..=n {
        vec.push(i);
    }

    let mut current = 0;
    while vec.len() > 1 {
        current = current + k - 1;
        let len = vec.len() as i32;
        if (current + 1) > len {
            current = current % len;
        }
        vec.remove(current as usize);
    }

    vec[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_the_winner_test() {
        assert_eq!(find_the_winner(5, 2), 3);
        assert_eq!(find_the_winner(6, 5), 1);
    }
}
