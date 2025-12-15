fn main() {
    let n = Some(5);
    let s = Some(String::from("hello"));
    let a: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    let mut temp = 0;
    match y {
        Some(i) => temp = i,
        None => {
            println!("no such thing");
        }
    }
    let sum = x + temp;
    println!("sum is {}", sum);

    let res = plus_one(n);
    match res {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    if let Some(value) = plus_one(Some(19)) {
        println!("{}", value);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
