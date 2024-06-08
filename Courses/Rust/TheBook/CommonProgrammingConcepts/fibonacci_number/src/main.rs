use std::io;

fn main() {
    println!("Please enter which number in the fibonacci sequence you would like to print.");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Unable to read the line.");

    let num: u32 = num
        .trim()
        .parse()
        .expect("The entered string is not a number");


    let mut prev_sum: u64 = 1;
    let mut curr_sum: u64 = 1;
    for _ in 3..=num {
        let temp: u64 = curr_sum;
        curr_sum = curr_sum + prev_sum;
        prev_sum = temp;
    }
    println!("The fibonacci number is {curr_sum}.");
    
}
