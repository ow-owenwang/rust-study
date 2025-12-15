fn main() {
    let s = String::from("hello world");

    let len = s.len();

    let hello = &s[0..5];
    let hello1 = &s[..5];
    let world = &s[6..11];
    let world1 = &s[6..len];
    let world2 = &s[6..];
    let txt1 = &s[0..len];
    let txt2 = &s[..];

    println!("{}", hello);
    println!("{}", hello1);
    println!("{}", world);
    println!("{}", world1);
    println!("{}", world2);
    println!("{}", txt1);
    println!("{}", txt2);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("{:?}", slice); // [2, 3]
    println!("{:?}", &[2,3,4]); // [2, 3, 4]

    assert_eq!(slice, &[2, 3]); // 切片的比较会逐个元素比对，只要元素值和顺序都一致，就认为相等。

    // 字符串字面量是切片
    let s1 = "Hello";
    let s2: &str = "Hello";
    assert_eq!(s1, s2);
}
