fn main() {
    let mut spaces = "    "; // spaces 是 &str 类型，可变
    spaces = spaces.len(); // 尝试将 usize 类型赋给 &str 类型
}
