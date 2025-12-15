fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", v.pop());
    println!("{:?}", v.len());
    println!("{:?}", v.capacity());
    v.push(100);
    v.push(100);
    println!("{:?}", v.capacity());

    println!("-------");

    for i in v.iter_mut() {
        *i += 10;
    }

    for i in 0..v.len() {
        println!("v[{:?}]={:?}", i, v[i]);
    }
}
