struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    x: Point<T>,
    y: Point<T>,
}

fn main() {
    let point1: Point<u32> = Point { x: 1, y: 2 };
    let point2: Point<u32> = Point { x: 2, y: 3 };
    let line = Line {
        x1: point1,
        y1: point2,
    };
}
