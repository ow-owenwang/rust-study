fn main() {
    let s = String::from("hello");
    let s1 = foo(s);
    println!("{}", s1);
}

fn foo(str: String) -> String {
    println!("{:?}", str);
    str
}
