// 枚举中使用范型
enum Option<T> {
    Some(T),
    None,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    // let Point { x: a, y: b } = p;
    println!("x: {}, y: {}", p.get_x(), p.get_y());
}
