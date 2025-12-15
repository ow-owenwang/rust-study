fn main() {
    let s2: String;
    {
        let s1 = String::from("hello");
        s2 = s1;
    }
    println!("{}", s2);
}
