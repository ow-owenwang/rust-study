/*
自动派生本质上是 trait 的一种自动实现方式

 */

// 下面的代码中，Rust 编译器会自动为 Point 类型生成 Debug、Clone 和 PartialEq 这三个 trait 的实现。
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let user1 = User {
        id: 1,
        name: "Alice".to_string(),
    };
    let user2 = user1.clone(); // 使用了 Clone 派生
    println!("{:?}", user2); // 使用了 Debug 派生
    assert_eq!(user1, user2); // 使用了 PartialEq 派生
}
