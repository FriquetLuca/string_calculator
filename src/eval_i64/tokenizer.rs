use super::token::{NativeFunction, Token};
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let current_char = self.expr.next();

        match current_char {
            Some('@') => Some(Token::Ans),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('|') => Some(Token::Bar),
            Some('!') => Some(Token::ExclamationMark),
            Some(',') => Some(Token::Comma),
            Some('%') => Some(Token::Modulo),
            Some('²') => Some(Token::Pow2),
            Some('³') => Some(Token::Pow3),
            Some('<') => {
                if self.expr.clone().take(1).collect::<String>() == "<" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::LeftShift)
                } else {
                    None
                }
            }
            Some('>') => {
                if self.expr.clone().take(1).collect::<String>() == ">" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::RightShift)
                } else {
                    None
                }
            }
            Some('0'..='9') => {
                let mut number = current_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_ascii_digit() {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<i64>().unwrap()))
            }
            Some('a') => {
                if self.expr.clone().take(3).collect::<String>() == "bs(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Abs))
                } else {
                    None
                }
            }
            Some('e') => {
                if self.expr.clone().take(4).collect::<String>() == "xp2(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Exp2))
                } else {
                    None
                }
            }
            Some('l') => {
                if self.expr.clone().take(2).collect::<String>() == "n(" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Ln))
                } else if self.expr.clone().take(3).collect::<String>() == "og(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Log))
                } else {
                    None
                }
            }
            Some('m') => {
                if self.expr.clone().take(3).collect::<String>() == "in(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Min))
                } else if self.expr.clone().take(3).collect::<String>() == "ax(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Max))
                } else {
                    None
                }
            }
            Some('p') => {
                if self.expr.clone().take(3).collect::<String>() == "ow(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Pow))
                } else {
                    None
                }
            }
            Some('r') => {
                if self.expr.clone().take(4).collect::<String>() == "oot(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Root))
                } else {
                    None
                }
            }
            Some('s') => {
                if self.expr.clone().take(4).collect::<String>() == "qrt(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sqrt))
                } else if self.expr.clone().take(4).collect::<String>() == "ign(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sign))
                } else if self.expr.clone().take(6).collect::<String>() == "ignum(" {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sign))
                } else if self.expr.clone().take(3).collect::<String>() == "gn(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sign))
                } else {
                    None
                }
            }
            None => Some(Token::Eof),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_number() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34))
    }
    #[test]
    fn test_no_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34))
    }
    #[test]
    fn test_sin_function() {
        let mut tokenizer = Tokenizer::new("pow(2, 3)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Pow)
        )
    }
}
