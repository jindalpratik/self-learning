use std::fmt::Debug;

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug)]
pub enum Status {
    Value(u32),
    Stop,
}

pub fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    dbg!(list_of_statuses);
}
