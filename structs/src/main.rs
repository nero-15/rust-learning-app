fn main() {
    // creating_instances_from_other_instances_with_struct_update_syntax()
    example_structs()
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn example_structs(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn creating_instances_from_other_instances_with_struct_update_syntax(){
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

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("{:?}", user3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
