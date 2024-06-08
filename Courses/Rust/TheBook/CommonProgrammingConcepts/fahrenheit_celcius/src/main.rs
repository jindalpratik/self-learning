use std::io;

fn main() {
    println!("This programme converts fahrenheit to celsius and vice versa.\nPlease choose 1 to convert to Fahrenheit and 2 to covert to Celsius.");

    let choice = loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Unable to read line");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            },
        };

        if choice == 1 || choice == 2 {
            break choice;
        } else {
            println!("Please enter either 1 or 2.");
        }
    };

    if choice == 1 {
        println!("Please enter the temperature in Celsius.");

        let mut c_num = String::new();
        io::stdin()
            .read_line(&mut c_num)
            .expect("Couldn't read the line.");
        let c_num = c_num.trim().parse().expect("The value entered is not a number.");

        let f_num = to_fahrenheit(c_num);

        println!("The value in fahrenheit is {f_num}°F");
    } else {

        println!("Please enter the temperature in Fahrenheit.");

        let mut f_num = String::new();
        io::stdin()
            .read_line(&mut f_num)
            .expect("Couldn't read the line.");
        let f_num = f_num.trim().parse().expect("The value entered is not a number.");

        let c_num = to_celsius(f_num);

        println!("The value in celsius is {c_num}°C");
    }

}

fn to_fahrenheit(c_num : f64) -> f64 {
    (c_num * (9.0/5.0)) + 32.0
}

fn to_celsius(f_num: f64) -> f64 {
    (f_num - 32.0) *(5.0/9.0)
}