use std::io::stdin;

fn main() {
    let mut no_test_case = String::new();
    stdin().read_line(&mut no_test_case).unwrap();
    for _ in 0..no_test_case.trim().parse().unwrap() {
        let mut test_case = String::new();
        stdin().read_line(&mut test_case).unwrap();
        for i in 1..=test_case.trim().parse().unwrap() {
            print!("{} ",i);
        }
        print!("\n");
    }
}
