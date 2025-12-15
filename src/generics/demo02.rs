struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

fn main() {
    let integer = Point {
        x: 5,
        y: 10,
        z: 15.21,
    };
    let float = Point {
        x: 1.0,
        y: 1.0,
        z: 8,
    };
}
