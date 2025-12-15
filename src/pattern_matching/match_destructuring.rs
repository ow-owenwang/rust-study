struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 5 };

    match point {
        Point { x: 0, y } => println!("On y-axis at y = {}", y),
        Point { x, y: 0 } => println!("On x-axis at x = {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
