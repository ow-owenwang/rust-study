/*
println!("{:?}", user1)
{:?} 是一种调试格式化标记，用于输出数据的“调试信息”。
这通常会调用实现了 Debug trait 的数据类型的 fmt::Debug 格式化方法。
适合用于输出开发者友好的调试信息，通常会包含更多的细节，且更容易直接识别数据的结构或内容。
使用这个格式需要保证数据类型实现了 Debug trait，大多数标准数据类型都实现了这个 trait。

println!("{}", user1)
{} 是一种常规的格式化标记，用于输出数据的“显示信息”。
这通常会调用实现了 Display trait 的数据类型的 fmt::Display 格式化方法。
适合用户友好的输出，通常会是更简洁、更清晰的展示信息，比如用户可读的字符串或其他简化的输出形式。
使用这个格式需要保证数据类型实现了 Display trait。
 */

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {}, Age: {}", self.name, self.age)
    }
}

fn main() {
    let user1 = User {
        name: String::from("Alice"),
        age: 30,
    };

    // 调试输出
    println!("{:?}", user1);  // 输出: User { name: "Alice", age: 30 }

    // 常规显示输出
    // println!("{}", user1) 调用时，Rust 会查找 User 类型的 fmt::Display 实现。
    println!("{}", user1);    // 输出: User: Alice, Age: 30
}
