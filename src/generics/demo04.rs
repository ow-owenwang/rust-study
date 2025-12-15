use std::cmp::PartialOrd;

struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Clone> Point<T> {
    fn largest(&self) -> T {
        if self.x > self.y {
            self.x.clone()
        } else {
            self.y.clone()
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p.largest());
    let p1 = Point { x: 1, y: 2 };
    println!("{:?}", p1.distance_from_origin());
}
