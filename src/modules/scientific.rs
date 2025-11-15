// # Big numbers.
// src/scientific.rs
// Scientific numbers base 10.
use std::fmt;

use crate::modules::{self, THOUSANDS};

/// # Scientific number.
/// Precision of the `body`, where a dot is placed after the first digit: `u32`.  
/// Precision of the `exponent` (of 10): `u32`.  
#[derive(Debug)]
pub struct Scientific {
    body: u32,
    exponent: u32,
}

impl Scientific {
    /// Construct a new `Scientific` number given a `body` and an `exponent`.
    pub fn new(body: u32, exponent: u32) -> Scientific {
        Scientific{
            body,
            exponent,
        }
    }

    /// Clone self.  
    pub fn clone(self: &Self) -> Scientific {
        Scientific::new(
            self.body,
            self.exponent
        )
    }

    /// Return the raw `u32` body.
    pub fn get_body(self: &Self) -> u32 {
        self.body
    }

    pub fn get_exponent(self: &Self) -> u32 {
        self.exponent
    }

    pub fn display_full(self: &Self) -> String {
        let mut string: String = self.body.to_string();
        for _ in 0..self.exponent {
            string.push('0');
        }

        modules::displays::thousand_separators(string, THOUSANDS)
    }

    /// Return a `String` of the body.
    /// Put the `DECIMALS` delimiter after the first digit, if there are more than one.
    pub fn display_body(self: &Self) -> String {
        let mut body: String = self.body.to_string();

        if body.len() > 1 {
            let digits: Vec<char> = modules::displays::digit_list(self.body);
            body = format!("{}{}{}", digits[0], modules::DECIMALS, &body[1..]);
        }

        body
    }
}

impl fmt::Display for Scientific {
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter, 
            "{}e{}", 
            modules::displays::thousand_separators(self.body.to_string(), THOUSANDS), 
            modules::displays::thousand_separators(self.exponent.to_string(), THOUSANDS),
        )
    }
}
