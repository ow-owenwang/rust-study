fn main() {
    for i in 0..5 {
        println!("{}", i);
    }

    println!("--------");

    for i in 0..=5 {
        println!("{}", i);
    }

    println!("--------");

    let arr = ["a", "b", "c"];
    for i in arr.iter() {
        println!("{}", i);
    }

    println!("--------");

    let mut arr1 = [1, 2, 3];
    for i in arr1.iter_mut() {
        *i *= 2;
    }
    for i in arr1.iter() {
        println!("{}", i);
    }
}
