use crate::utils::superscript_digit_to_digit;

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
            Some('!') => Some(Token::ExclamationMark),
            Some(',') => Some(Token::Comma),
            Some('%') => Some(Token::Modulo),
            Some('π') => Some(Token::Pi),
            Some('⌊') => Some(Token::LeftFloor),
            Some('⌋') => Some(Token::RightFloor),
            Some('⌈') => Some(Token::LeftCeiling),
            Some('⌉') => Some(Token::RightCeiling),
            Some('°') => Some(Token::DegToRad),
            Some('.') => {
                let next_char = self.expr.peek()?;
                if next_char.is_ascii_digit() {
                    let mut number = "0".to_string();
                    number.push(current_char?);
                    while let Some(next_char) = self.expr.peek() {
                        if next_char.is_ascii_digit() {
                            number.push(self.expr.next()?);
                        } else {
                            break;
                        }
                    }
                    Some(Token::Num(number.parse::<f64>().unwrap()))
                } else {
                    None
                }
            }
            Some('⁰'..='⁹') => {
                let current_char = current_char?;
                let mut number = superscript_digit_to_digit(&current_char)
                    .map(|c| c.to_string())
                    .unwrap_or_default();
                while let Some(next_char) = self.expr.peek() {
                    if let Some(next_char) = superscript_digit_to_digit(next_char) {
                        self.expr.next();
                        number.push(next_char);
                    } else {
                        break;
                    }
                }
                Some(Token::Superscript(number.parse::<f64>().unwrap()))
            }
            Some('0'..='9') => {
                let mut number = current_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_ascii_digit() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('a') => match self.expr.clone().take(6).collect::<String>().as_str() {
                "rsinh(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Arsinh))
                }
                "rcosh(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Arcosh))
                }
                "rtanh(" => {
                    self.expr.by_ref().take(5).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Artanh))
                }
                _ => match self.expr.clone().take(5).collect::<String>().as_str() {
                    "tan2(" => {
                        self.expr.by_ref().take(4).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Atan2))
                    }
                    "sinh(" => {
                        self.expr.by_ref().take(4).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Arsinh))
                    }
                    "cosh(" => {
                        self.expr.by_ref().take(4).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Arcosh))
                    }
                    "tanh(" => {
                        self.expr.by_ref().take(4).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Artanh))
                    }
                    _ => match self.expr.clone().take(4).collect::<String>().as_str() {
                        "sin(" => {
                            self.expr.by_ref().take(3).for_each(drop);
                            Some(Token::ExplicitFunction(NativeFunction::Asin))
                        }
                        "cos(" => {
                            self.expr.by_ref().take(3).for_each(drop);
                            Some(Token::ExplicitFunction(NativeFunction::Acos))
                        }
                        "tan(" => {
                            self.expr.by_ref().take(3).for_each(drop);
                            Some(Token::ExplicitFunction(NativeFunction::Atan))
                        }
                        _ => match self.expr.clone().take(3).collect::<String>().as_str() {
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
                    },
                },
            },
            Some('c') => match self.expr.clone().take(4).collect::<String>().as_str() {
                "eil(" => {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Ceil))
                }
                "osh(" => {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Cosh))
                }
                _ => match self.expr.clone().take(3).collect::<String>().as_str() {
                    "os(" => {
                        self.expr.by_ref().take(2).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Cos))
                    }
                    _ => None,
                },
            },
            Some('f') => {
                if self.expr.clone().take(5).collect::<String>() == "loor(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Floor))
                } else {
                    None
                }
            }
            Some('e') => {
                if self.expr.clone().take(4).collect::<String>() == "xp2(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Exp2))
                } else if self.expr.clone().take(3).collect::<String>() == "xp(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Exp))
                } else {
                    Some(Token::E)
                }
            }
            Some('l') => {
                if self.expr.clone().take(3).collect::<String>() == "og(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Log))
                } else if self.expr.clone().take(2).collect::<String>() == "n(" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Ln))
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
            Some('p') => {
                if self.expr.clone().take(3).collect::<String>() == "ow(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Pow))
                } else if self.expr.clone().take(1).collect::<String>() == "i" {
                    self.expr.by_ref().take(1).for_each(drop);
                    Some(Token::Pi)
                } else {
                    None
                }
            }
            Some('r') => {
                if self.expr.clone().take(5).collect::<String>() == "ound(" {
                    self.expr.by_ref().take(4).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Round))
                } else if self.expr.clone().take(4).collect::<String>() == "oot(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Root))
                } else if self.expr.clone().take(2).collect::<String>() == "ad" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::RadToDeg)
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
                    "inh(" => {
                        self.expr.by_ref().take(3).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Sinh))
                    }
                    "qrt(" => {
                        self.expr.by_ref().take(3).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Sqrt))
                    }
                    "ign(" => {
                        self.expr.by_ref().take(3).for_each(drop);
                        Some(Token::ExplicitFunction(NativeFunction::Sign))
                    }
                    _ => match self.expr.clone().take(3).collect::<String>().as_str() {
                        "in(" => {
                            self.expr.by_ref().take(2).for_each(drop);
                            Some(Token::ExplicitFunction(NativeFunction::Sin))
                        }
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
                } else if self.expr.clone().take(4).collect::<String>() == "anh(" {
                    self.expr.by_ref().take(3).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Tanh))
                } else if self.expr.clone().take(3).collect::<String>() == "an(" {
                    self.expr.by_ref().take(2).for_each(drop);
                    Some(Token::ExplicitFunction(NativeFunction::Tan))
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
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_superscript_number() {
        let mut tokenizer = Tokenizer::new("⁰¹²³⁴⁵⁶⁷⁸⁹");
        assert_eq!(tokenizer.next().unwrap(), Token::Superscript(123456789.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    fn test_decimal_number_omit_zero() {
        let mut tokenizer = Tokenizer::new(".5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(0.5))
    }
    #[test]
    fn test_e_const() {
        let mut tokenizer = Tokenizer::new("e");
        assert_eq!(tokenizer.next().unwrap(), Token::E)
    }
    #[test]
    fn test_pi_const() {
        let mut tokenizer = Tokenizer::new("π");
        assert_eq!(tokenizer.next().unwrap(), Token::Pi)
    }
    #[test]
    fn test_pi_const2() {
        let mut tokenizer = Tokenizer::new("pi");
        assert_eq!(tokenizer.next().unwrap(), Token::Pi)
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
    fn test_deg_to_rad_operator() {
        let mut tokenizer = Tokenizer::new("°");
        assert_eq!(tokenizer.next().unwrap(), Token::DegToRad)
    }
    #[test]
    fn test_rad_to_deg_operator() {
        let mut tokenizer = Tokenizer::new("rad");
        assert_eq!(tokenizer.next().unwrap(), Token::RadToDeg)
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
    fn test_exclamation_mark_operator() {
        let mut tokenizer = Tokenizer::new("!");
        assert_eq!(tokenizer.next().unwrap(), Token::ExclamationMark)
    }
    #[test]
    fn test_sin_function() {
        let mut tokenizer = Tokenizer::new("sin(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sin)
        )
    }
    #[test]
    fn test_cos_function() {
        let mut tokenizer = Tokenizer::new("cos(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Cos)
        )
    }
    #[test]
    fn test_tan_function() {
        let mut tokenizer = Tokenizer::new("tan(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Tan)
        )
    }
    #[test]
    fn test_sinh_function() {
        let mut tokenizer = Tokenizer::new("sinh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sinh)
        )
    }
    #[test]
    fn test_cosh_function() {
        let mut tokenizer = Tokenizer::new("cosh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Cosh)
        )
    }
    #[test]
    fn test_tanh_function() {
        let mut tokenizer = Tokenizer::new("tanh(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Tanh)
        )
    }
    #[test]
    fn test_asin_function() {
        let mut tokenizer = Tokenizer::new("asin(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Asin)
        )
    }
    #[test]
    fn test_acos_function() {
        let mut tokenizer = Tokenizer::new("acos(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Acos)
        )
    }
    #[test]
    fn test_atan_function() {
        let mut tokenizer = Tokenizer::new("atan(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Atan)
        )
    }
    #[test]
    fn test_atan2_function() {
        let mut tokenizer = Tokenizer::new("atan2(.14159, 2.1415)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Atan2)
        )
    }
    #[test]
    fn test_asinh_function() {
        let mut tokenizer = Tokenizer::new("asinh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Arsinh)
        )
    }
    #[test]
    fn test_acosh_function() {
        let mut tokenizer = Tokenizer::new("acosh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Arcosh)
        )
    }
    #[test]
    fn test_atanh_function() {
        let mut tokenizer = Tokenizer::new("atanh(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Artanh)
        )
    }
    #[test]
    fn test_arsinh_function() {
        let mut tokenizer = Tokenizer::new("arsinh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Arsinh)
        )
    }
    #[test]
    fn test_arcosh_function() {
        let mut tokenizer = Tokenizer::new("arcosh(3.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Arcosh)
        )
    }
    #[test]
    fn test_artanh_function() {
        let mut tokenizer = Tokenizer::new("artanh(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Artanh)
        )
    }
    #[test]
    fn test_ln_function() {
        let mut tokenizer = Tokenizer::new("ln(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Ln)
        )
    }
    #[test]
    fn test_log_function() {
        let mut tokenizer = Tokenizer::new("log(.14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Log)
        )
    }
    #[test]
    fn test_pow_function() {
        let mut tokenizer = Tokenizer::new("pow(.14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Pow)
        )
    }
    #[test]
    fn test_root_function() {
        let mut tokenizer = Tokenizer::new("root(.14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Root)
        )
    }
    #[test]
    fn test_mod_function() {
        let mut tokenizer = Tokenizer::new("mod(2.14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Mod)
        )
    }
    #[test]
    fn test_sqrt_function() {
        let mut tokenizer = Tokenizer::new("sqrt(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sqrt)
        )
    }
    #[test]
    fn test_exp_function() {
        let mut tokenizer = Tokenizer::new("exp(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Exp)
        )
    }
    #[test]
    fn test_exp2_function() {
        let mut tokenizer = Tokenizer::new("exp2(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Exp2)
        )
    }
    #[test]
    fn test_abs_function() {
        let mut tokenizer = Tokenizer::new("abs(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Abs)
        )
    }
    #[test]
    fn test_sign_function() {
        let mut tokenizer = Tokenizer::new("sign(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
        )
    }
    #[test]
    fn test_sign2_function() {
        let mut tokenizer = Tokenizer::new("sgn(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
        )
    }
    #[test]
    fn test_sign3_function() {
        let mut tokenizer = Tokenizer::new("signum(.14159)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Sign)
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
    #[test]
    fn test_min_function() {
        let mut tokenizer = Tokenizer::new("min(.14159,2)");
        assert_eq!(
            tokenizer.next().unwrap(),
            Token::ExplicitFunction(NativeFunction::Min)
        )
    }
    #[test]
    fn test_max_function() {
        let mut tokenizer = Tokenizer::new("max(.14159,2)");
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
}
