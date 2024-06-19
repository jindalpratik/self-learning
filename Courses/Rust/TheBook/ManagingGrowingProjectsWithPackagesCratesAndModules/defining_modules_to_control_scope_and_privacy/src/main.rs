use crate::garden::vegetables::Aparagus;

pub mod garden;

fn main() {
    let plant = Aparagus {};

    println!("I am growing {plant:?}!");
}