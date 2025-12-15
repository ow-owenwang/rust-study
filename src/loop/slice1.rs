fn main() {
    let s = String::from("hello world");

    let s1 = &s[0..5];
    let s1 = &s[0..=4];
    let s1 = &s[..=4];
    let s1 = &s[..5];

    println!("s1: {}", s1);

    let s2 = &s[6..11];
    let s2 = &s[6..];
    let s2 = &s[..];
    println!("s2: {}", s2);

    let s3 = String::from("你好");
    let s4 = &s3[1..3];
    println!("s4: {}", s4); // 报错
}