use std::{iter::Peekable, str::Chars};

use super::superscript_digit_to_digit;

pub fn deserialize_superscript_number(current_char: &char, expr: &mut Peekable<Chars>) -> String {
    let mut number = superscript_digit_to_digit(current_char)
        .map(|c| c.to_string())
        .unwrap_or_default();
    while let Some(next_char) = expr.peek() {
        if let Some(next_char) = superscript_digit_to_digit(next_char) {
            expr.next();
            number.push(next_char);
        } else {
            break;
        }
    }
    number
}
