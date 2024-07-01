use super::token::{NativeFunction, Token};
use core::str::FromStr;
use rust_decimal::Decimal;
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
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some(',') => Some(Token::Comma),
            Some('%') => Some(Token::Modulo),
            Some('⌊') => Some(Token::LeftFloor),
            Some('⌋') => Some(Token::RightFloor),
            Some('⌈') => Some(Token::LeftCeiling),
            Some('⌉') => Some(Token::RightCeiling),
            Some('0'..='9') => {
                let mut number = current_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_ascii_digit() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Num(Decimal::from_str(&number).unwrap()))
            }
            Some('a') => match self.expr.clone().take(3).collect::<String>().as_str() {
                "bs(" => {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Abs))
                }
                "vg(" => {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Avg))
                }
                _ => None,
            },
            Some('c') => match self.expr.clone().take(4).collect::<String>().as_str() {
                "eil(" => {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Ceil))
                }
                _ => None,
            },
            Some('f') => {
                if self.expr.clone().take(5).collect::<String>() == "loor(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Floor))
                } else {
                    None
                }
            }
            Some('m') => match self.expr.clone().take(6).collect::<String>().as_str() {
                "edian(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Med))
                }
                _ => match self.expr.clone().take(3).collect::<String>().as_str() {
                    "in(" => {
                        self.expr.by_ref().take(2).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Min))
                    }
                    "ax(" => {
                        self.expr.by_ref().take(2).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Max))
                    }
                    "od(" => {
                        self.expr.by_ref().take(2).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Mod))
                    }
                    "ed(" => {
                        self.expr.by_ref().take(2).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Med))
                    }
                    _ => None,
                },
            },
            Some('r') => {
                if self.expr.clone().take(5).collect::<String>() == "ound(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Round))
                } else {
                    None
                }
            }
            Some('s') => match self.expr.clone().take(6).collect::<String>().as_str() {
                "ignum(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sign))
                }
                _ => match self.expr.clone().take(4).collect::<String>().as_str() {
                    "ign(" => {
                        self.expr.by_ref().take(3).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Sign))
                    }
                    _ => match self.expr.clone().take(3).collect::<String>().as_str() {
                        "gn(" => {
                            self.expr.by_ref().take(2).for_each(drop);
                            Some(Token::ExplicitFunction(NativeFunction::Sign))
                        }
                        _ => None,
                    },
                },
            },
            Some('t') => {
                if self.expr.clone().take(8).collect::<String>() == "runcate(" {
                    self.expr.by_ref().take(7).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Truncate))
                } else if self.expr.clone().take(5).collect::<String>() == "runc(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Truncate))
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
        assert_eq!(tokenizer.next().unwrap(), Token::Num(Decimal::new(34, 0)))
    }
    #[test]
    fn test_left_parenthesis_operator() {
        let mut tokenizer = Tokenizer::new("(");
        assert_eq!(tokenizer.next().unwrap(), Token::LeftParen)
    }
    #[test]
    fn test_right_parenthesis_operator() {
        let mut tokenizer = Tokenizer::new(")");
        assert_eq!(tokenizer.next().unwrap(), Token::RightParen)
    }
    #[test]
    fn test_left_floor_operator() {
        let mut tokenizer = Tokenizer::new("⌊");
        assert_eq!(tokenizer.next().unwrap(), Token::LeftFloor)
    }
    #[test]
    fn test_right_floor_operator() {
        let mut tokenizer = Tokenizer::new("⌋");
        assert_eq!(tokenizer.next().unwrap(), Token::RightFloor)
    }
    #[test]
    fn test_left_ceil_operator() {
        let mut tokenizer = Tokenizer::new("⌈");
        assert_eq!(tokenizer.next().unwrap(), Token::LeftCeiling)
    }
    #[test]
    fn test_right_ceil_operator() {
        let mut tokenizer = Tokenizer::new("⌉");
        assert_eq!(tokenizer.next().unwrap(), Token::RightCeiling)
    }
    #[test]
    fn test_ans_operator() {
        let mut tokenizer = Tokenizer::new("@");
        assert_eq!(tokenizer.next().unwrap(), Token::Ans)
    }
    #[test]
    fn test_comma_operator() {
        let mut tokenizer = Tokenizer::new(",");
        assert_eq!(tokenizer.next().unwrap(), Token::Comma)
    }
    #[test]
    fn test_add_operator() {
        let mut tokenizer = Tokenizer::new("+");
        assert_eq!(tokenizer.next().unwrap(), Token::Add)
    }
    #[test]
    fn test_subtract_operator() {
        let mut tokenizer = Tokenizer::new("-");
        assert_eq!(tokenizer.next().unwrap(), Token::Subtract)
    }
    #[test]
    fn test_multiply_operator() {
        let mut tokenizer = Tokenizer::new("*");
        assert_eq!(tokenizer.next().unwrap(), Token::Multiply)
    }
    #[test]
    fn test_divide_operator() {
        let mut tokenizer = Tokenizer::new("/");
        assert_eq!(tokenizer.next().unwrap(), Token::Divide)
    }
    #[test]
    fn test_modulo_operator() {
        let mut tokenizer = Tokenizer::new("%");
        assert_eq!(tokenizer.next().unwrap(), Token::Modulo)
    }
    #[test]
    fn test_mod_function() {
        let mut tokenizer = Tokenizer::new("mod(20,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Mod)
        )
    }
    #[test]
    fn test_abs_function() {
        let mut tokenizer = Tokenizer::new("abs(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Abs)
        )
    }
    #[test]
    fn test_sign_function() {
        let mut tokenizer = Tokenizer::new("sign(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
        )
    }
    #[test]
    fn test_sign2_function() {
        let mut tokenizer = Tokenizer::new("sgn(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
        )
    }
    #[test]
    fn test_sign3_function() {
        let mut tokenizer = Tokenizer::new("signum(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
        )
    }
    #[test]
    fn test_min_function() {
        let mut tokenizer = Tokenizer::new("min(14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Min)
        )
    }
    #[test]
    fn test_max_function() {
        let mut tokenizer = Tokenizer::new("max(14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Max)
        )
    }
    #[test]
    fn test_avg_function() {
        let mut tokenizer = Tokenizer::new("avg(10,20)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Avg)
        )
    }
    #[test]
    fn test_median_function() {
        let mut tokenizer = Tokenizer::new("med(10,20)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Med)
        )
    }
    #[test]
    fn test_median_function2() {
        let mut tokenizer = Tokenizer::new("median(10,20)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Med)
        )
    }
    #[test]
    fn test_truncate_function() {
        let mut tokenizer = Tokenizer::new("truncate(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Truncate)
        )
    }
    #[test]
    fn test_truncate_function2() {
        let mut tokenizer = Tokenizer::new("trunc(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Truncate)
        )
    }
    #[test]
    fn test_floor_function() {
        let mut tokenizer = Tokenizer::new("floor(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Floor)
        )
    }
    #[test]
    fn test_ceil_function() {
        let mut tokenizer = Tokenizer::new("ceil(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Ceil)
        )
    }
    #[test]
    fn test_round_function() {
        let mut tokenizer = Tokenizer::new("round(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Round)
        )
    }
}
