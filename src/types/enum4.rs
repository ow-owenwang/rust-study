fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number); // Some(5)
    println!("{:?}", some_string); // Some("a string")
    println!("{:?}", absent_number); // None

    let names = vec!["Alice", "Bob", "Charlie"];
    let found = names.iter().find(|&&name| name == "Bob");
    // let found = names.iter().find(|name| **name == "Bob");
    // let found = names.iter().copied().find(|&name| name == "Bob");

    match found {
        Some(name) => println!("Found: {}", name),
        None => println!("Not found."),
    }
}
