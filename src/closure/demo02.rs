fn main() {
    let name = String::from("Rust");

    // FnOnce（会将 name 移动）
    let consume = move || {
        println!("Goodbye, {}", name);
    };
    consume();
}
