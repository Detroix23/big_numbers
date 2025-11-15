// # Big numbers.
// src/scientific.rs
// Scientific numbers base 10.
use std::fmt;

use crate::modules;

/// # Scientific number.
/// Precision of the `body`, where a dot is placed after the first digit: `u32`.  
/// Precision of the `exponent` (of 10): `u32`.  
#[derive(Debug)]
pub struct Scientific {
    body: u32,
    exponent: u32,
}

impl Scientific {
    pub fn new(body: u32, exponent: u32) -> Scientific {
        Scientific{
            body,
            exponent,
        }
    }

    /// Return a `String` of the body.
    /// Put the `DECIMALS` delimiter after the first digit, if there are more than one.
    pub fn display_body(self: &Self) -> String {
        let mut body: String = self.body.to_string();

        if body.len() > 1 {
            let digits = modules::displays::digit_list(self.body);
            body = format!("{}{}{}", digits[0], modules::DECIMALS, &body[1..]);
        }

        body
    }
}

impl fmt::Display for Scientific {
    fn fmt(self: &Self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}e{}", self.display_body(), self.exponent)
    }
}
