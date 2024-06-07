use std::io;

fn main() {
    let _guess: u32 = "42".parse().expect("This is an error");

    let float_64 = 2.0;

    let _float_32 : f32 = 2.0;

    println!("{float_64}");

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of y is: {five_hundred}");
    println!("The value of y is: {six_point_four}");
    println!("The value of y is: {one}");

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1,2,3,4,5];

    println!("Please enter an array idex");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("The entered string is not a number.");

    let element = a[index];

    println!("The vale of the element at {index} is: {element}");
}
