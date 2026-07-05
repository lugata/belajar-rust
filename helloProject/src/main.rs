#![allow(warnings)]
fn main() {
    let rect = (10, 20);

    struct Book {
        title: String,
        author: String,
        year: u32,
        pages: u32,
        available: bool,
    }

    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
    }

    let mut user1 = User {
        username: String::from("john_doe"),
        email: String::from("john.doe@example.com"),
        sign_in_count: 0,
        active: true,
    };

    // user1.email = String::from("another@email.com");
    println!("User1 Email: {}", user1.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }

    let user2 = User {
        email: String::from("jane.smith@example.com"),
        ..user1
    };
    println!("User2 Email: {}", user2.email);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let green = Color(0, 255, 0);

    struct AlwaysEqual;

    let subject = AlwaysEqual;
}
