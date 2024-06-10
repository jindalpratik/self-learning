fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@example.com");

    println!("The user is active: {}",user1.active);
    println!("The username of the user is {}",user1.username);
    println!("The email of the user is {}",user1.email);
    println!("The user has signed in {} times",user1.sign_in_count);

    
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1 // This also makes user1 unusable as we are moving values here while assigning as username is a String
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn _build_user(email: String, username: String) -> User {
    // Returns using build init shorthand where the values are same as the struct parametera making it
    // as a result we don't need to write username: username and can just write username.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}