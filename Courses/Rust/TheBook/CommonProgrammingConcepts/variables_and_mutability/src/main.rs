fn main() {
    // Have to explicitly state that the value is mutable.
    let mut x = 5;
    println!("The value of x is {x}");
    x = 106;
    println!("The value of x is {x}");

    // Const alows us to have hardcode non immutable values in out code.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours.");

    // Shadowing in rust.
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // It is also uselfull for changing the datatype of variable while still keeping the same varible name
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The number of spaces are {spaces}");
}
