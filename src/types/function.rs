fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}

fn main() {
    let ret = add(1, 2);
    println!("{}", ret);

    let x = plus_or_minus(5);

    println!("The value of x is: {}", x);
}
