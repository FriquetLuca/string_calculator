use super::token::{NativeFunction, Token};
use crate::utils::deserialize_superscript_number;
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
            Some('!') => Some(Token::ExclamationMark),
            Some('&') => Some(Token::Ampersand),
            Some('|') => Some(Token::Bar),
            Some(',') => Some(Token::Comma),
            Some('%') => Some(Token::Modulo),
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
            Some('⁰') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('¹') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('²') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('³') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁴') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁵') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁶') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁷') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁸') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
            Some('⁹') => Some(Token::Superscript(
                deserialize_superscript_number(&current_char?, &mut self.expr)
                    .parse::<i64>()
                    .unwrap(),
            )),
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
            Some('e') => {
                if self.expr.clone().take(4).collect::<String>() == "xp2(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Exp2))
                } else if self.expr.clone().take(3).collect::<String>() == "xp(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Exp))
                } else {
                    None
                }
            }
            Some('g') => {
                if self.expr.clone().take(3).collect::<String>() == "cd(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Gcd))
                } else {
                    None
                }
            }
            Some('l') => match self.expr.clone().take(3).collect::<String>().as_str() {
                "og(" => {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Log))
                }
                "cm(" => {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Lcm))
                }
                _ => match self.expr.clone().take(2).collect::<String>().as_str() {
                    "n(" => {
                        self.expr.by_ref().take(1).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Ln))
                    }
                    "b(" => {
                        self.expr.by_ref().take(1).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Lb))
                    }
                    _ => None,
                },
            },
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
            Some('s') => match self.expr.clone().take(6).collect::<String>().as_str() {
                "ignum(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Sign))
                }
                _ => match self.expr.clone().take(4).collect::<String>().as_str() {
                    "qrt(" => {
                        self.expr.by_ref().take(3).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Sqrt))
                    }
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
    fn test_superscript_number() {
        let mut tokenizer = Tokenizer::new("⁰¹²³⁴⁵⁶⁷⁸⁹");
        assert_eq!(tokenizer.next().unwrap(), Token::Superscript(123456789))
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
    fn test_left_shift_operator() {
        let mut tokenizer = Tokenizer::new("<<");
        assert_eq!(tokenizer.next().unwrap(), Token::LeftShift)
    }
    #[test]
    fn test_right_shift_operator() {
        let mut tokenizer = Tokenizer::new(">>");
        assert_eq!(tokenizer.next().unwrap(), Token::RightShift)
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
    fn test_caret_operator() {
        let mut tokenizer = Tokenizer::new("^");
        assert_eq!(tokenizer.next().unwrap(), Token::Caret)
    }
    #[test]
    fn test_ampersand_operator() {
        let mut tokenizer = Tokenizer::new("&");
        assert_eq!(tokenizer.next().unwrap(), Token::Ampersand)
    }
    #[test]
    fn test_bar_operator() {
        let mut tokenizer = Tokenizer::new("|");
        assert_eq!(tokenizer.next().unwrap(), Token::Bar)
    }
    #[test]
    fn test_exclamation_mark_operator() {
        let mut tokenizer = Tokenizer::new("!");
        assert_eq!(tokenizer.next().unwrap(), Token::ExclamationMark)
    }
    #[test]
    fn test_ln_function() {
        let mut tokenizer = Tokenizer::new("ln(10)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Ln)
        )
    }
    #[test]
    fn test_log_function() {
        let mut tokenizer = Tokenizer::new("log(20,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Log)
        )
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
    fn test_pow_function() {
        let mut tokenizer = Tokenizer::new("pow(20,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Pow)
        )
    }
    #[test]
    fn test_sqrt_function() {
        let mut tokenizer = Tokenizer::new("sqrt(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sqrt)
        )
    }
    #[test]
    fn test_exp_function() {
        let mut tokenizer = Tokenizer::new("exp(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Exp)
        )
    }
    #[test]
    fn test_exp2_function() {
        let mut tokenizer = Tokenizer::new("exp2(14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Exp2)
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
    fn test_gcd_function() {
        let mut tokenizer = Tokenizer::new("gcd(10,20)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Gcd)
        )
    }
    #[test]
    fn test_lcm_function() {
        let mut tokenizer = Tokenizer::new("lcm(10,20)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Lcm)
        )
    }
}
