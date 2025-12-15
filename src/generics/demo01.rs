use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn main() {
    let a: f64 = 3.14;
    let b: f64 = 1.2;
    println!("{}", largest(a, b));
}
