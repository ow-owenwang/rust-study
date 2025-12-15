use std::mem;

fn main() {
    unsafe {
        // 字节序
        // 大端序 小端序
        let a = [0u8, 1u8, 0u8, 0u8];
        let b: u32 = mem::transmute(a);
        println!("{}", b);
    }
}
