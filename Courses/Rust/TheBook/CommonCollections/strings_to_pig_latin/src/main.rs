use std::io;

fn main() {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Unable to read input.");

    let pig_latin_string = string_to_pig_latin(string);

    println!("The string in pig latin is {}.", pig_latin_string);
}

fn string_to_pig_latin(s: String) -> String {
    let (first_letter, suffix) = s.trim().split_at(1);
    if "aeiou".contains(&first_letter.to_lowercase()) {
        format!("{}{}-hay", first_letter, suffix)
    } else {
        format!("{}-{}ay", suffix, first_letter)
    }
}
