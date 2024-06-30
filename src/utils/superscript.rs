pub fn superscript_digit_to_digit(current_char: &char) -> Option<char> {
  match current_char {
    '⁰' => Some('0'),
    '¹' => Some('1'),
    '²' => Some('2'),
    '³' => Some('3'),
    '⁴' => Some('4'),
    '⁵' => Some('5'),
    '⁶' => Some('6'),
    '⁷' => Some('7'),
    '⁸' => Some('8'),
    '⁹' => Some('9'),
    _ => {
      None
    }
  }
}
