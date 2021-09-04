fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    user1.email = String::from("anotheremail@example.com");
    println!("{:?}", user1);


    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(
        String::from("tokatoka@example.com"),
        String::from("tokatoka")
    );
    println!("{:?}", user2);
}
