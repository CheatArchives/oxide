use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct OxideError {
    message: String,
}

impl OxideError {
    pub fn new(msg: &str) -> OxideError {
        OxideError {
            message: msg.to_owned(),
        }
    }
}

impl Error for OxideError {}

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
