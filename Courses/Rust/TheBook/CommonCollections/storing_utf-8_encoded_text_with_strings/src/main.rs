fn main() {

    let mut _s = String::new();

    let data = "initial content";

    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("The string is {}",s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("The value of s3 is {s3}, s2 is {s2}.");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("The value of s is {s}.");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    dbg!(s);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    dbg!(s);

    for c in "Зд".chars() {
        println!("{c}");
    }
    
    for b in "Зд".bytes() {
        println!("{b}");
    }
    
}
