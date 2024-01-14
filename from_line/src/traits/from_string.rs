use std::io::Error;

pub trait FromString: Sized {
    fn from_string(s: &str) -> Result<Self, Error>;
}
