fn main() {
    let mut user1 = build_user(
        String::from("someusername@email.com"),
        String::from("someusername"),
    );

    user1.email = String::from("someusername@example.com");
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
