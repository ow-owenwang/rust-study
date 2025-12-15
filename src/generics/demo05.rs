struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> std::fmt::Display for Point<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn show<T: std::fmt::Display>(a: T){
    println!("{}", a);
}

// 简写
fn show1(a: impl std::fmt::Display){
    println!("{}", a);
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let point1 = Point { x: 1, y: 2 };
    println!("{}", point);
    show(point);
    show1(point1);
}
