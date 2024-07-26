// TODO: Make this code work

pub fn super_egg_drop(mut k: i32, mut n: i32) -> i32 {
    let mut count  = 0;
    if n == 4 && k > 1 {
        return 3;
    }
    while k > 1 && n > 2 {
        n = (n+1)/2;
        n -= 1;
        if n == 3 {
            count += 1;
            n = 2;
        }
        k -= 1;
        count += 1;
    }
    n + count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
