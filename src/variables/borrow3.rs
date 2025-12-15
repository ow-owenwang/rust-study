fn main() {
    let s = String::from("hello");
    echo(&s);
    println!("{}", s);
}

// 借用
fn echo(s: &String) {
    println!("{}", s);
}
