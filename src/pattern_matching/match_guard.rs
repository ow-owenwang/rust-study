fn main() {
    let x = Some(5);

    match x {
        Some(n) if n > 3 => println!("Greater than 3"),
        Some(_) => println!("Not greater than 3"),
        None => println!("None"),
    }

    let number = Some(8);

    match number {
        Some(n) if n < 5 => println!("Less than 5: {}", n),
        Some(n) if n >= 5 && n <= 10 => println!("Between 5 and 10: {}", n),
        Some(n) => println!("Greater than 10: {}", n),
        None => println!("No number"),
    }
}
