use std::io::Error;

use crate::traits::from_string::FromString;

impl FromString for char {
    fn from_string(s: &str) -> Result<Self, Error> {
        Ok(s.chars().next().unwrap_or(' '))
    }
}

impl FromString for Option<char> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        Ok(Some(s.chars().next().unwrap_or(' ')))
    }
}

impl FromString for String {
    fn from_string(s: &str) -> Result<Self, Error> {
        Ok(s.to_string())
    }
}

impl FromString for Option<String> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }
        Ok(Some(s.to_string()))
    }
}
