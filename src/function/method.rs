#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Point {
        Point { x, y }
    }
}

fn main() {
    let p = Point::new(1, 2);
    println!("p = {:?}", p);
}
