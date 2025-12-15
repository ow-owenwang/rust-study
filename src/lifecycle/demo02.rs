struct A<'a> {
    name: &'a str,
}

fn main() {
    let s = String::from("hello");
    let a = A { name: &s };
    println!("{}", a.name);
}
