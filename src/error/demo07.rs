#[derive(Debug)]
pub enum Error {
    IO(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err.kind())
    }
}

fn do_read_file() -> Result<(), Error> {
    let data = std::fs::read("12.txt")?;
    let data_str = std::str::from_utf8(&data).unwrap();
    println!("{:?}", data_str);
    Ok(())
}

fn main() {
    do_read_file().unwrap();
}
