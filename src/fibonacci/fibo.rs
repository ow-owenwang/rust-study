fn fibo(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    // Calculate fibo(10)
    println!("fibo(10) = {}", fibo(10));
}