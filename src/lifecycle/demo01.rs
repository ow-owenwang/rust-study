// fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn foo<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn bar<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let r = String::from("hello");
//     r.as_str()
// }

fn main() {
    let s1 = String::from("hi");
    let s2 = String::from("world");
    let r = longest(s1.as_str(), s2.as_str());
    println!("{}", r);

    // let s3 = bar(s1.as_str(), s2.as_str()); // 报错
}
