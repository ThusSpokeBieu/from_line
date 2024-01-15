use rust_decimal::Decimal;
use std::io::Error;

use crate::{regex::regex_const::only_zero_regex, traits::from_string::FromString};

impl FromString for Decimal {
    fn from_string(s: &str) -> Result<Self, Error> {
        match s.parse::<Decimal>() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Decimal",
            )),
        }
    }
}

impl FromString for Option<Decimal> {
    fn from_string(s: &str) -> Result<Self, Error> {
        if s.is_empty() || only_zero_regex().is_match(s) {
            return Ok(None);
        }

        let len = s.len();

        let value;

        match len {
            1 => value = vec!["0.0", s].concat(),
            2 => value = vec!["0.", s].concat(),
            _ => {
                let (integral, fractional) = s.split_at(len - 2);
                value = vec![integral, ".", fractional].concat();
            }
        }

        match value.parse::<Decimal>() {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Decimal",
            )),
        }
    }
}
