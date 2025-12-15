use std::cmp::Ordering;

fn main() {
    let x: u32 = 5;
    let y: u32 = 10;

    let result = x.cmp(&y);
    println!("{:?}", result); // Less

    fn compare(a: &u32, b: &u32) -> Ordering {
        a.cmp(b)
    }

    let result1 = compare(&x, &y);
    println!("{:?}", result1); // Less
}
