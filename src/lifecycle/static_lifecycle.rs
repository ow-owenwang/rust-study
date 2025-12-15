use std::fmt::Display;

fn foo<'a, T: Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
    println!("ann: {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let ann = 42;
    let r = foo(s1.as_str(), s2.as_str(), ann);
    println!("{}", r);
}
