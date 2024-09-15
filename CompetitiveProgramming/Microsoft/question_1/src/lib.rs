use std::cmp::max;

pub fn solution(A: &mut Vec<i32>, B: &mut Vec<i32>) -> i32 {
    
    let m = A.len();
    let vec = vec![A.clone(), B.clone()];

    let mut ans = i32::MAX;
    let mut mins = i32::MAX;
    let maxi = i32::MIN;

    solve(&vec, 1, (m - 1) as i32, &mut ans, 0, &mut mins, maxi);
    ans
}


fn solve(arr: &Vec<Vec<i32>>, n: i32, m: i32, ans: &mut i32, sum: i32, mins: &mut i32, maxi: i32) {
    if n < 0 || m < 0 {
        return;
    }
    if n == 0 && m == 0 {
        let sum = sum + arr[n as usize][m as usize];
        if sum < *mins {
            *mins = sum;
            *ans = maxi;
        } else if sum == *mins {
            if maxi > *ans {
                *ans = maxi;
            }
        }
        return;
    }

    let maxi = max(arr[n as usize][m as usize], maxi);
    // down
    solve(arr, n - 1, m, ans, sum + arr[n as usize][m as usize], mins, maxi);
    // right
    solve(arr, n, m - 1, ans, sum + arr[n as usize][m as usize], mins, maxi);
}
