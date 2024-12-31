fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@email.com"),
        sign_in_count: 4,
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
