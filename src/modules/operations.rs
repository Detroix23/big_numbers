// # Big numbers.
// src/modules/operations.rs

use crate::modules::scientific;

/// Return if the addition of `a` and `b` would overflow.
pub fn will_addition_overflow(a: &scientific::Scientific, b: &scientific::Scientific) -> bool {
    let max: u32 = std::u32::MAX - a.get_body();

    b.get_body() > max
}

/// Add 2 `Scientific` notations.
pub fn addition(a: scientific::Scientific, b: scientific::Scientific) -> scientific::Scientific {
    if a.get_exponent() == b.get_exponent() {
        let exponent: u32 = a.get_exponent();
        
        if will_addition_overflow(&a, &b) {
            return scientific::Scientific::new(
                a.get_body() / 10 + b.get_body() / 10,
                exponent + 1
            )

        } else {
            return scientific::Scientific::new(
                a.get_body() + b.get_body(),
                exponent
            )
        }
    } else {
        todo!("Implement different exponent additions. ")

    }
}