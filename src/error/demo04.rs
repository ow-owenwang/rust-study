use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("1.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("成功 {}", s),
        Err(e) => println!("失败 {}", e),
    }
}
