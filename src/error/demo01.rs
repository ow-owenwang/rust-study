use std::fs::File;

fn main() {
    // panic!("----crash here----");

    /*let f = File::open("1.txt");
    let res = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };*/

    // let f = File::open("1.txt").unwrap();

    let f = File::open("1.txt").expect("file not found");

}
