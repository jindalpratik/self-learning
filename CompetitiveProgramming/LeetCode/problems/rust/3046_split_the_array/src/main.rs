fn main() {
    let nums = vec![1, 1, 2, 2, 3, 4];
    println!("{}", is_possible_to_split(nums));
}

pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut num1 = Vec::new();
    let mut num2 = Vec::new();
    for i in &nums {
        if num1.contains(i) {
            if num2.contains(i) {
                return false;
            } else {
                num2.push(*i);
            }
        } else {
            num1.push(*i);
        }
    }
    true
}
