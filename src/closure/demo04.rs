fn main() {
    let name = String::from("Alice");

    // Fn（不可变借用）
    let greet = || {
        println!("Hello, {}", name);
    };

    greet();
    greet(); // 可多次调用
}
