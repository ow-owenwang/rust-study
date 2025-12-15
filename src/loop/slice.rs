fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &arr[0..3];
    println!("slice[0] = {}, len = {}", slice[0], slice.len());

    let slice2 = &arr[3..5];
    println!("slice2[0] = {}, len = {}", slice2[0], slice2.len());

    if slice2.len() == 0 {
        println!("empty");
    }

    if slice2.is_empty() {
        println!("empty");
    }

    let mut arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let slice3 = &mut arr1[..];
    slice3[0] = 9;
    println!("arr1[0] = {}", arr1[0]);
}
