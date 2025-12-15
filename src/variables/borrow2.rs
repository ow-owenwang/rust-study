fn main() {
    let s = String::from("hello");
    echo(s);
    println!("{}", s); // 报错
}

fn echo(s: String) {
    println!("{}", s);
}
