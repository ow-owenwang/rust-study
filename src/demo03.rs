#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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

    let user2 = build_user(String::from("11@qq.com"), String::from("hi"));
    println!("{:?}", user2);
}
