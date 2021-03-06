struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let mut user1 = build_user(String::from("user@example.com"),
                           String::from("username"));
    user1.email = String::from("another@example.com");
    println!("{}", user1.email);
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{}", user2.sign_in_count);
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", origin.0);


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}