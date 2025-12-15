fn main() {
    let mut count = 0;

    // FnMut（需要可变借用）
    let mut inc = || {
        count += 1;
        println!("Count: {}", count);
    };

    inc();
    inc();
}
