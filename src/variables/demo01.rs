fn main() {
    // 不可变可以带来安全性，但是丧失了灵活性和性能
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
