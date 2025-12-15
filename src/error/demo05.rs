fn main() {
    assert_eq!(4, Some(4));
}

fn divide_by_three(x: u32) -> u32 {
    for i in 0.. {
        if 3 * i < i {
            println!("u32 overflow");
        }
        if x < 3 * i {
            return i - 1;
        }
    }
    unreachable!();
}

fn add(a: u32, b: u32) -> u32 {
    unimplemented!()
}
