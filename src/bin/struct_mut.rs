struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    user1 = User {
        email: String::from("foo@example.com"),
        username: String::from("bar"),
        active: true,
        sign_in_count: 1,
    };
}
