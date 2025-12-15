#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
fn main() {
    let user1 = build_user("11@qq.com".to_string(), "hi".to_string());
    println!("{:?}", user1);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!("{:?}", user3);
}