fn main() {
    let mut user1 = build_user(
        String::from("someusername@email.com"),
        String::from("someusername"),
    );

    let mut user2 = User {
        email: String::from("anotheruser@email.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
