fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{}", s2);
        let s3 = s2.clone();
        println!("{}", s2);
        println!("{}", s3);
    }
}
