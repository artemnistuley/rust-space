struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email: {}", user1.email);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        ..user1
    };
    user2.email = String::from("anotheremail@example.com");

    let user3 = create_user(
        String::from("test@example.com"), 
        String::from("test user")
    );
    println!("username {}", user3.username);


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn create_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email, // email: email
        sign_in_count: 1,
    }
}
