use question_1::solution;

fn main() {
    let mut a = vec![1, 2, 1, 1, 1, 4];
    let mut b = vec![1, 1, 1, 3, 1, 1];

    let ans = solution(&mut a, &mut b);
    println!("{}", ans);
}