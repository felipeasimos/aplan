use std::io::Write;

pub(crate) fn to_file(filename: Option<&str>, s: String) {
    match filename {
        Some("-") | None => write!(std::io::stdout(), "{}", s).unwrap(),
        Some(f) => std::fs::write(f, s).unwrap()
    }
}
