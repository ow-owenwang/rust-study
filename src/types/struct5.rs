#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // dbg! 是 Rust 的一个调试宏，它会将表达式的 值和代码位置 打印出来，并返回表达式的结果。
        height: 50,
    };

    dbg!(&rect1); // 注意这里使用 &rect1 是为了避免发生所有权转移（借用而非移动）。
}
