fn main() {
    let v = vec![1, 2, 3];
    let output = v
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    println!("[{}]", output);

    println!("{}", join_numbers(&v, ", "));
}

fn join_numbers(v: &[i32], sep: &str) -> String {
    v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(sep)
}