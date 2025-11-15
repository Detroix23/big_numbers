/// # Big numbers.
/// src/displays.rs

/// Give the length, the number of digit of the given `number`.  
/// Use a conversion to a string.
pub fn number_length(number: &u32) -> usize {
    number.to_string().len()
}

/// Convert a given `digit` to a `char`.     
/// Panic if not a digit.    
pub fn digit_to_char(digit: u32) -> char {
    let string: String = digit.to_string();
    
    if string.len() != 1 {
        panic!("(X) - modules.displays.digit_to_char - Not a digit ({}).", digit);
    }

    string.as_bytes()[0] as char
}

/// Convert an `u32` to a `Vec<char>`.
pub fn digit_list(number: u32) -> Vec<char> {
    let mut cloned: u32 = number.clone();
    let mut list: Vec<char> = Vec::new();

    for _ in 0..number_length(&number) {
        list.insert(0, digit_to_char(cloned % 10));
        cloned /= 10;
    }

    list
}