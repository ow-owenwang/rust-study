#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let name = String::from("John");
    let age = 20;
    let p = Person { name, age };
    println!("{}, {}", p.name, p.age);

    let p1 = Person {
        name: String::from("Doe"),
        ..p
    };

    println!("{:?}", p1);
    println!("{:#?}", p1);

}
