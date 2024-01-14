use crate::traits::from_string::FromString;
use std::io::Error;

impl FromString for u8 {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<u8>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u8")),
        }
    }
}

impl FromString for Option<u8> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse::<u8>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u8")),
        }
    }
}

impl FromString for u16 {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<u16>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u16")),
        }
    }
}

impl FromString for Option<u16> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse::<u16>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u16")),
        }
    }
}

impl FromString for u32 {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<u32>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u32")),
        }
    }
}

impl FromString for Option<u32> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse::<u32>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid u32")),
        }
    }
}

impl FromString for i8 {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<i8>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid i8")),
        }
    }
}

impl FromString for Option<i8> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse::<i8>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid i8")),
        }
    }
}

impl FromString for i32 {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<i32>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid i32")),
        }
    }
}

impl FromString for Option<i32> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() {
            return Ok(None);
        }

        match s.parse::<i32>() {
            Ok(v) => Ok(Some(v)),
            Err(_) => Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid i32")),
        }
    }
}
