#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let i1 = IpAddr::V4(String::from("127.0.0.1"));
    let i2 = IpAddr::V6(String::from("::1"));

    println!("{:?}", i1);
    println!("{:?}", i2);
}
