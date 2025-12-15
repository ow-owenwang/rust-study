trait GetName {
    fn get_name(&self) -> &String;
}

trait GetAge {
    fn get_age(&self) -> u32;
}

// 写法一
// fn print_information<T: GetName + GetAge>(t: T) {
//     println!("name: {}, age: {}", t.get_name(), t.get_age());
// }

// 写法二
fn print_information<T>(t: T)
where
    T: GetName + GetAge,
{
    println!("name: {}, age: {}", t.get_name(), t.get_age());
}

struct Person {
    name: String,
    age: u32,
}

impl GetName for Person {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl GetAge for Person {
    fn get_age(&self) -> u32 {
        self.age
    }
}

// 要求返回值实现了GetAge特征
fn produce_item_with_age() -> impl GetAge {
    Person {
        name: String::from("Rust"),
        age: 30,
    }
}

fn main() {
    let p = Person {
        name: String::from("Emma"),
        age: 18,
    };

    print_information(p);

    let p1 = produce_item_with_age();
    println!("{}", p1.get_age());
}
