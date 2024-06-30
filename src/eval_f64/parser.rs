use std::fmt;

use super::ast::Node;
use super::token::{NativeFunction, OperPrec, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    previous_token: Option<Token>,
    placeholder: f64,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str, placeholder: Option<f64>) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
            previous_token: None,
            placeholder: placeholder.unwrap_or_default(),
        })
    }
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
}
impl<'a> Parser<'a> {
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.previous_token = Some(self.current_token.clone());
        self.current_token = next_token;
        Ok(())
    }
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::Eof {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }
    fn function_static_arguments(&mut self, n: i32) -> Result<Vec<Node>, ParseError> {
        self.get_next_token()?;
        self.check_paren(Token::LeftParen)?;
        let mut args = Vec::new();
        for i in 0..n {
            let arg_expr = self.generate_ast(OperPrec::DefaultZero)?;
            args.push(arg_expr);
            if i < n - 1 {
                self.check_paren(Token::Comma)?;
            }
        }
        self.check_paren(Token::RightParen)?;
        Ok(args)
    }
    fn function_arguments(&mut self) -> Result<Vec<Node>, ParseError> {
        self.find_item_list(Token::LeftParen, Token::RightParen, OperPrec::DefaultZero)
    }
    fn find_item_list(
        &mut self,
        start_token: Token,
        end_token: Token,
        oper_prec: OperPrec,
    ) -> Result<Vec<Node>, ParseError> {
        self.get_next_token()?;
        self.check_paren(start_token)?;
        let mut args = Vec::new();
        loop {
            if args.is_empty() && (end_token == self.current_token) {
                self.get_next_token()?;
                break;
            }
            let arg_expr = self.generate_ast(oper_prec.clone())?;
            args.push(arg_expr);
            if Token::Comma == self.current_token {
                self.get_next_token()?;
            } else if end_token == self.current_token {
                self.get_next_token()?;
                break;
            } else {
                return Err(ParseError::InvalidOperator(format!(
                    "Expected either {:?} or {:?}, got {:?}",
                    Token::Comma,
                    end_token,
                    self.current_token
                )));
            }
        }
        Ok(args)
    }
    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Ans => {
                self.get_next_token()?;
                Ok(Node::Number(self.placeholder))
            }
            Token::ExplicitFunction(current_function) => {
                let current_function = match current_function {
                    NativeFunction::Abs => {
                        Node::Abs(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Floor => {
                        Node::Floor(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Ceil => {
                        Node::Ceil(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Round => {
                        Node::Round(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Sin => {
                        Node::Sin(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Cos => {
                        Node::Cos(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Tan => {
                        Node::Tan(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Sinh => {
                        Node::Sinh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Cosh => {
                        Node::Cosh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Tanh => {
                        Node::Tanh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Asin => {
                        Node::Asin(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Acos => {
                        Node::Acos(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Atan => {
                        Node::Atan(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Arsinh => {
                        Node::Arsinh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Arcosh => {
                        Node::Arcosh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Artanh => {
                        Node::Artanh(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Sqrt => {
                        Node::Sqrt(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Exp => {
                        Node::Exp(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Exp2 => {
                        Node::Exp2(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Ln => {
                        Node::Ln(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Sign => {
                        Node::Sign(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Truncate => {
                        Node::Truncate(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Atan2 => {
                        let args = self.function_static_arguments(2)?;
                        Node::Atan2(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Mod => {
                        let args = self.function_static_arguments(2)?;
                        Node::Modulo(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Pow => {
                        let args = self.function_static_arguments(2)?;
                        Node::Pow(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Log => {
                        let args = self.function_static_arguments(2)?;
                        Node::Log(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Min => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            return Err(ParseError::UnableToParse(
                                "There's no arguments in the min function".to_string(),
                            ));
                        }
                        Node::Min(args)
                    }
                    NativeFunction::Max => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            return Err(ParseError::UnableToParse(
                                "There's no arguments in the max function".to_string(),
                            ));
                        }
                        Node::Max(args)
                    }
                };
                self.implicit_multiply(current_function)
            }
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Add => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(expr)
            }
            Token::Num(i) => {
                self.get_next_token()?;
                self.implicit_multiply(Node::Number(i))
            }
            Token::Pi => {
                self.get_next_token()?;
                Ok(Node::Number(std::f64::consts::PI))
            }
            Token::E => {
                self.get_next_token()?;
                Ok(Node::Number(std::f64::consts::E))
            }
            Token::LeftParen => self.get_enclosed_elements_with_impl_mult(
                OperPrec::DefaultZero,
                Token::RightParen,
                |expr| expr,
            ),
            Token::Bar => self.get_enclosed_elements_with_impl_mult(
                OperPrec::DefaultZero,
                Token::Bar,
                |expr| Node::Abs(Box::new(expr)),
            ),
            Token::LeftFloor => self.get_enclosed_elements_with_impl_mult(
                OperPrec::DefaultZero,
                Token::RightFloor,
                |expr| Node::Floor(Box::new(expr)),
            ),
            Token::LeftCeiling => self.get_enclosed_elements_with_impl_mult(
                OperPrec::DefaultZero,
                Token::RightCeiling,
                |expr| Node::Ceil(Box::new(expr)),
            ),
            _ => Err(ParseError::UnableToParse(
                "Unknown parsing token for parsing number".to_string(),
            )),
        }
    }
    fn implicit_multiply(&mut self, node: Node) -> Result<Node, ParseError> {
        if (self.current_token == Token::LeftParen)
            || (self.current_token == Token::LeftCeiling)
            || (self.current_token == Token::LeftFloor)
            || matches!(self.current_token, Token::ExplicitFunction(_))
            || matches!(self.current_token, Token::Num(_))
        {
            let right = self.generate_ast(OperPrec::MulDiv)?;
            return Ok(Node::Multiply(Box::new(node), Box::new(right)));
        }
        Ok(node)
    }
    fn get_enclosed_elements_with_impl_mult(
        &mut self,
        oper_prec: OperPrec,
        end_token: Token,
        get_node: fn(Node) -> Node,
    ) -> Result<Node, ParseError> {
        self.get_next_token()?;
        let expr = self.generate_ast(oper_prec)?;
        self.check_paren(end_token)?;
        self.implicit_multiply(get_node(expr))
    }
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::ExclamationMark => {
                self.get_next_token()?;
                self.implicit_multiply(Node::Factorial(Box::new(left_expr)))
            }
            Token::DegToRad => {
                self.get_next_token()?;
                Ok(Node::Multiply(
                    Box::new(left_expr),
                    Box::new(Node::Number(0.017453292519943295)),
                ))
            }
            Token::RadToDeg => {
                self.get_next_token()?;
                Ok(Node::Multiply(
                    Box::new(left_expr),
                    Box::new(Node::Number(57.2957795131)),
                ))
            }
            Token::Pow2 => {
                self.get_next_token()?;
                Ok(Node::Pow2(Box::new(left_expr)))
            }
            Token::Pow3 => {
                self.get_next_token()?;
                Ok(Node::Pow3(Box::new(left_expr)))
            }
            Token::Modulo => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Modulo(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter a valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            self::ParseError::UnableToParse(e) => e.clone(),
            self::ParseError::InvalidOperator(e) => e.clone(),
        };
        write!(f, "Error in evaluating {}", message)
    }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval_f64::ast::Node::*;

    #[test]
    fn test_pi() {
        let mut parser = Parser::new("pi", None).unwrap();
        let expected = Number(std::f64::consts::PI);
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_e() {
        let mut parser = Parser::new("e", None).unwrap();
        let expected = Number(std::f64::consts::E);
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_negative() {
        let mut parser = Parser::new("-1", None).unwrap();
        let expected = Negative(Box::new(Number(1.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_add() {
        let mut parser = Parser::new("1+2", None).unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_subtract() {
        let mut parser = Parser::new("1-2", None).unwrap();
        let expected = Subtract(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_multiply() {
        let mut parser = Parser::new("1*2", None).unwrap();
        let expected = Multiply(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_divide() {
        let mut parser = Parser::new("1/2", None).unwrap();
        let expected = Divide(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo() {
        let mut parser = Parser::new("1%2", None).unwrap();
        let expected = Modulo(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_caret() {
        let mut parser = Parser::new("1^2", None).unwrap();
        let expected = Caret(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exclamation_mark() {
        let mut parser = Parser::new("1!", None).unwrap();
        let expected = Factorial(Box::new(Number(1.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_deg_to_rad() {
        let mut parser = Parser::new("1°", None).unwrap();
        let expected = Multiply(
            Box::new(Number(1.0)),
            Box::new(Number(0.017453292519943295)),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_rad_to_deg() {
        let mut parser = Parser::new("1rad", None).unwrap();
        let expected = Multiply(Box::new(Number(1.0)), Box::new(Number(57.2957795131)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_pow2() {
        let mut parser = Parser::new("1²", None).unwrap();
        let expected = Pow2(Box::new(Number(1.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_pow3() {
        let mut parser = Parser::new("1³", None).unwrap();
        let expected = Pow3(Box::new(Number(1.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_bar() {
        let mut parser = Parser::new("|5|", None).unwrap();
        let expected = Abs(Box::new(Number(5.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_floor() {
        let mut parser = Parser::new("⌊5.25⌋", None).unwrap();
        let expected = Floor(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ceil() {
        let mut parser = Parser::new("⌈5.25⌉", None).unwrap();
        let expected = Ceil(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo_function() {
        let mut parser = Parser::new("mod(3,2)", None).unwrap();
        let expected = Modulo(Box::new(Number(3.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_pow_function() {
        let mut parser = Parser::new("pow(3,2)", None).unwrap();
        let expected = Pow(Box::new(Number(3.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_log_function() {
        let mut parser = Parser::new("log(3,2)", None).unwrap();
        let expected = Log(Box::new(Number(3.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_abs_function() {
        let mut parser = Parser::new("abs(5.25)", None).unwrap();
        let expected = Abs(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_floor_function() {
        let mut parser = Parser::new("floor(5.25)", None).unwrap();
        let expected = Floor(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ceil_function() {
        let mut parser = Parser::new("ceil(5.25)", None).unwrap();
        let expected = Ceil(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_round_function() {
        let mut parser = Parser::new("round(5.25)", None).unwrap();
        let expected = Round(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_truncate_function() {
        let mut parser = Parser::new("truncate(5.25)", None).unwrap();
        let expected = Truncate(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sqrt_function() {
        let mut parser = Parser::new("sqrt(5.25)", None).unwrap();
        let expected = Sqrt(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp_function() {
        let mut parser = Parser::new("exp(5.25)", None).unwrap();
        let expected = Exp(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp2_function() {
        let mut parser = Parser::new("exp2(5.25)", None).unwrap();
        let expected = Exp2(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ln_function() {
        let mut parser = Parser::new("ln(5.25)", None).unwrap();
        let expected = Ln(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sign_function() {
        let mut parser = Parser::new("sign(5.25)", None).unwrap();
        let expected = Sign(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sin_function() {
        let mut parser = Parser::new("sin(5.25)", None).unwrap();
        let expected = Sin(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_cos_function() {
        let mut parser = Parser::new("cos(5.25)", None).unwrap();
        let expected = Cos(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_tan_function() {
        let mut parser = Parser::new("tan(5.25)", None).unwrap();
        let expected = Tan(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sinh_function() {
        let mut parser = Parser::new("sinh(5.25)", None).unwrap();
        let expected = Sinh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_cosh_function() {
        let mut parser = Parser::new("cosh(5.25)", None).unwrap();
        let expected = Cosh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_tanh_function() {
        let mut parser = Parser::new("tanh(5.25)", None).unwrap();
        let expected = Tanh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_asin_function() {
        let mut parser = Parser::new("asin(5.25)", None).unwrap();
        let expected = Asin(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_acos_function() {
        let mut parser = Parser::new("acos(5.25)", None).unwrap();
        let expected = Acos(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_atan_function() {
        let mut parser = Parser::new("atan(5.25)", None).unwrap();
        let expected = Atan(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_atan2_function() {
        let mut parser = Parser::new("atan2(5.25,7.8)", None).unwrap();
        let expected = Atan2(Box::new(Number(5.25)), Box::new(Number(7.8)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_arsinh_function() {
        let mut parser = Parser::new("arsinh(5.25)", None).unwrap();
        let expected = Arsinh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_arcosh_function() {
        let mut parser = Parser::new("arcosh(5.25)", None).unwrap();
        let expected = Arcosh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_artanh_function() {
        let mut parser = Parser::new("artanh(5.25)", None).unwrap();
        let expected = Artanh(Box::new(Number(5.25)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function() {
        let mut parser = Parser::new("min(3)", None).unwrap();
        let expected = Min(vec![Number(3.0)]);
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function2() {
        let mut parser = Parser::new("min(2,3,5)", None).unwrap();
        let expected = Min(vec![Number(2.0), Number(3.0), Number(5.0)]);
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function() {
        let mut parser = Parser::new("max(3)", None).unwrap();
        let expected = Max(vec![Number(3.0)]);
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function2() {
        let mut parser = Parser::new("max(2,3,5)", None).unwrap();
        let expected = Max(vec![Number(2.0), Number(3.0), Number(5.0)]);
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
