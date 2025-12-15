struct Pair(i32, f32);

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug)]
struct Unit;
fn main() {
    let pair = Pair(100, 2.5);
    println!("{}", pair.0);

    let p = Person {
        name: String::from("John"),
        age: 20,
    };
    println!("name = {} age = {}", p.name, p.age);
    println!("{:?}", p);

    let unit = Unit;
    println!("{:?}", unit);
}
