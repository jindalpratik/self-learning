fn main() {
    let some_option_value:Option<i32> = None;

    let Some(x) = some_option_value;

    if let Some(x) = some_option_value {
        println!("{x}");
    }

    if let x = 5 {
        println!("{x}");
    };
}
