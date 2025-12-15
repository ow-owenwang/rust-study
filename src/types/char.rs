fn main() {
    // 字符
    // Rust 的字符不仅仅是 ASCII，所有的 Unicode 值都可以作为 Rust 字符
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&g));
}
