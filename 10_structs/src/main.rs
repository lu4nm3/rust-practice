fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let user1 = User {
        email: String::from("user1@test.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };

    println!("user1's email is: {}", user1.email);



    let mut user1 = User {
        email: String::from("user1@test.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("test@test.com");

    println!("user1's new email is: {}", user1.email);



    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };



    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

