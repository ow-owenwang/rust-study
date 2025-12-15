pub trait GetInfo {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

// 默认实现
trait SchoolName {
    fn get_school_name(&self) -> String {
        String::from("牛栏山中学")
    }
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInfo for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl SchoolName for Student {}

impl GetInfo for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl SchoolName for Teacher {
    fn get_school_name(&self) -> String {
        String::from("光明中学")
    }
}

/*fn foo(item: impl GetInfo) {
    println!("{}", item.get_name());
}*/
// 简写
fn foo<T: GetInfo>(item: T) {
    println!("{}", item.get_name());
}

fn main() {
    let s = Student {
        name: String::from("Alex"),
        age: 18,
    };
    let t = Teacher {
        name: String::from("John"),
        age: 18,
        subject: String::from(""),
    };
    println!("name = {}, age = {}", s.get_name(), s.get_age());
    println!("name = {}, age = {}", t.get_name(), t.get_age());

    // foo(s);
    let result = s.get_school_name();
    println!("{}", result);
    let result1 = t.get_school_name();
    println!("{}", result1);
}
