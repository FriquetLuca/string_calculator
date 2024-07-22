use num_complex::Complex;

use super::ast::Node;
use super::token::{NativeFunction, Token};
use super::tokenizer::Tokenizer;
use crate::utils::{OperatorCategory, ParseError};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    previous_token: Option<Token>,
    placeholder: Complex<f64>,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str, placeholder: Option<Complex<f64>>) -> Result<Self, ParseError> {
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
        let ast = self.generate_ast(OperatorCategory::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.previous_token = Some(self.current_token.clone());
        self.current_token = next_token;
        Ok(())
    }
    fn generate_ast(&mut self, oper_prec: OperatorCategory) -> Result<Node, ParseError> {
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
            let arg_expr = self.generate_ast(OperatorCategory::DefaultZero)?;
            args.push(arg_expr);
            if i < n - 1 {
                self.check_paren(Token::Comma)?;
            }
        }
        self.check_paren(Token::RightParen)?;
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
                    NativeFunction::Lb => {
                        Node::Lb(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Pow => {
                        let args = self.function_static_arguments(2)?;
                        Node::Pow(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Root => {
                        let args = self.function_static_arguments(2)?;
                        Node::Root(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Log => {
                        let args = self.function_static_arguments(2)?;
                        Node::Log(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                };
                self.implicit_multiply(current_function)
            }
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperatorCategory::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Add => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperatorCategory::Negative)?;
                Ok(expr)
            }
            Token::Num(i) => {
                self.get_next_token()?;
                self.implicit_multiply(Node::Number(i))
            }
            Token::Pi => {
                self.get_next_token()?;
                Ok(Node::Number(Complex::new(std::f64::consts::PI, 0.0)))
            }
            Token::E => {
                self.get_next_token()?;
                Ok(Node::Number(Complex::new(std::f64::consts::E, 0.0)))
            }
            Token::LeftParen => self.get_enclosed_elements_with_impl_mult(
                OperatorCategory::DefaultZero,
                Token::RightParen,
                |expr| expr,
            ),
            _ => Err(ParseError::UnableToParse(
                "Unknown parsing token for parsing number".to_string(),
            )),
        }
    }
    fn implicit_multiply(&mut self, node: Node) -> Result<Node, ParseError> {
        if (self.current_token == Token::LeftParen)
            || matches!(self.current_token, Token::ExplicitFunction(_))
            || matches!(self.current_token, Token::Num(_))
        {
            let right = self.generate_ast(OperatorCategory::Multiplicative)?;
            return Ok(Node::Multiply(Box::new(node), Box::new(right)));
        }
        Ok(node)
    }
    fn get_enclosed_elements_with_impl_mult(
        &mut self,
        oper_prec: OperatorCategory,
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
                let right_expr = self.generate_ast(OperatorCategory::Additive)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorCategory::Additive)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorCategory::Multiplicative)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorCategory::Multiplicative)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorCategory::Power)?;
                Ok(Node::Pow(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::DegToRad => {
                self.get_next_token()?;
                Ok(Node::Multiply(
                    Box::new(left_expr),
                    Box::new(Node::Number(Complex::new(0.017453292519943295, 0.0))),
                ))
            }
            Token::RadToDeg => {
                self.get_next_token()?;
                Ok(Node::Multiply(
                    Box::new(left_expr),
                    Box::new(Node::Number(Complex::new(57.2957795131, 0.0))),
                ))
            }
            Token::Superscript(script) => {
                self.get_next_token()?;
                Ok(Node::Pow(
                    Box::new(left_expr),
                    Box::new(Node::Number(script)),
                ))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter a valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval_complex::ast::Node::*;

    #[test]
    fn test_pi() {
        let mut parser = Parser::new("pi", None).unwrap();
        let expected = Number(Complex::new(std::f64::consts::PI, 0.0));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_e() {
        let mut parser = Parser::new("e", None).unwrap();
        let expected = Number(Complex::new(std::f64::consts::E, 0.0));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_negative() {
        let mut parser = Parser::new("-1", None).unwrap();
        let expected = Negative(Box::new(Number(Complex::new(1.0, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_add() {
        let mut parser = Parser::new("1+2", None).unwrap();
        let expected = Add(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_subtract() {
        let mut parser = Parser::new("1-2", None).unwrap();
        let expected = Subtract(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_multiply() {
        let mut parser = Parser::new("1*2", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_divide() {
        let mut parser = Parser::new("1/2", None).unwrap();
        let expected = Divide(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_caret() {
        let mut parser = Parser::new("1^2", None).unwrap();
        let expected = Pow(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_deg_to_rad() {
        let mut parser = Parser::new("1°", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(0.017453292519943295, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_rad_to_deg() {
        let mut parser = Parser::new("1rad", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Complex::new(1.0, 0.0))),
            Box::new(Number(Complex::new(57.2957795131, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_pow_function() {
        let mut parser = Parser::new("pow(3,2)", None).unwrap();
        let expected = Pow(
            Box::new(Number(Complex::new(3.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_log_function() {
        let mut parser = Parser::new("log(3,2)", None).unwrap();
        let expected = Log(
            Box::new(Number(Complex::new(3.0, 0.0))),
            Box::new(Number(Complex::new(2.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_abs_function() {
        let mut parser = Parser::new("abs(5.25)", None).unwrap();
        let expected = Abs(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sqrt_function() {
        let mut parser = Parser::new("sqrt(5.25)", None).unwrap();
        let expected = Sqrt(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp_function() {
        let mut parser = Parser::new("exp(5.25)", None).unwrap();
        let expected = Exp(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp2_function() {
        let mut parser = Parser::new("exp2(5.25)", None).unwrap();
        let expected = Exp2(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ln_function() {
        let mut parser = Parser::new("ln(5.25)", None).unwrap();
        let expected = Ln(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_lb_function() {
        let mut parser = Parser::new("lb(5.25)", None).unwrap();
        let expected = Lb(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sin_function() {
        let mut parser = Parser::new("sin(5.25)", None).unwrap();
        let expected = Sin(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_cos_function() {
        let mut parser = Parser::new("cos(5.25)", None).unwrap();
        let expected = Cos(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_tan_function() {
        let mut parser = Parser::new("tan(5.25)", None).unwrap();
        let expected = Tan(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sinh_function() {
        let mut parser = Parser::new("sinh(5.25)", None).unwrap();
        let expected = Sinh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_cosh_function() {
        let mut parser = Parser::new("cosh(5.25)", None).unwrap();
        let expected = Cosh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_tanh_function() {
        let mut parser = Parser::new("tanh(5.25)", None).unwrap();
        let expected = Tanh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_asin_function() {
        let mut parser = Parser::new("asin(5.25)", None).unwrap();
        let expected = Asin(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_acos_function() {
        let mut parser = Parser::new("acos(5.25)", None).unwrap();
        let expected = Acos(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_atan_function() {
        let mut parser = Parser::new("atan(5.25)", None).unwrap();
        let expected = Atan(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_arsinh_function() {
        let mut parser = Parser::new("arsinh(5.25)", None).unwrap();
        let expected = Arsinh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_arcosh_function() {
        let mut parser = Parser::new("arcosh(5.25)", None).unwrap();
        let expected = Arcosh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_artanh_function() {
        let mut parser = Parser::new("artanh(5.25)", None).unwrap();
        let expected = Artanh(Box::new(Number(Complex::new(5.25, 0.0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_implicit_mul_prts() {
        let mut parser = Parser::new("(2)(3)", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Complex::new(2.0, 0.0))),
            Box::new(Number(Complex::new(3.0, 0.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
