fn main() {
    // 可变引用，同一时间内最多只能有一个可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let s1_ref = &mut s;
    let s2_ref = &mut s; // second mutable borrow occurs here
    println!("{:?}", s1_ref);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
