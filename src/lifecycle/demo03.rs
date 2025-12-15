struct StuA<'a> {
    name: &'a str,
}

impl<'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }

    // 为什么不用写生命周期？
    fn do_something2(&self, s: &str) -> &str {
        self.name
    }

    // 为什么这里不标注生命周期就报错？
    fn do_something3(&self, s: &str) -> &str {
        s
    }
}

fn main() {
    let s = String::from("hello");
    let a = StuA { name: &s };
    println!("{}", a.do_something());
    let s2 = String::from("world");
    println!("{}", a.do_something2(&s2));
    let s3 = String::from("h");
    println!("{}", a.do_something3(&s3));
}
