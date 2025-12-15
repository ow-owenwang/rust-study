fn main() {
    let maybe_name = Some("Alice");

    // match 写法
    match maybe_name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("No name found."),
    }

    // if let 写法（更简洁）
    if let Some(name) = maybe_name {
        println!("Hello again, {}!", name);
    } else {
        println!("Still no name.");
    }
}
