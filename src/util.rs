use std::io::Write;

use crate::prelude::Error;

pub static DEFAULT_FILENAME : &str =".aplan.ap";

pub fn to_file(filename: Option<&str>, s: String) -> Result<(), Error> {
    match filename {
        Some("-") | None => write!(std::io::stdout(), "{}", s).or_else(|_| Err(Error::FileWrite("-".to_string()))),
        Some(f) => std::fs::write(f, s).or_else(|_| Err(Error::FileWrite(f.to_string())))
    }
}

pub fn from_file(filename: Option<&str>) -> Result<String, Error> {
    let filename = filename.unwrap_or(DEFAULT_FILENAME);
    std::fs::read_to_string(filename)
        .or_else(|_| Err(Error::FileRead(filename.to_string())))
}
