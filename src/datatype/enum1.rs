fn main() {
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    enum IpAddr {
        IPv4(u8, u8, u8, u8), // 元组
        IPv6(
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
            u8,
        ),
    }

    // 枚举通常与match模式匹配一起使用
    let localhost: IpAddr = IpAddr::IPv4(127, 0, 0, 1);
    match localhost {
        IpAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {}
    }
}
