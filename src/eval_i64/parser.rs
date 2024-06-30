use std::fmt;
use std::sync::Arc;

use super::ast::Node;
use super::token::{NativeFunction, OperPrec, Token};
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    previous_token: Option<Token>,
    placeholder: i64,
}
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str, placeholder: Option<i64>) -> Result<Self, ParseError> {
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
                    NativeFunction::Sqrt => {
                        Node::Sqrt(Box::new(self.function_static_arguments(1)?[0].clone()))
                    }
                    NativeFunction::Root => {
                        let args = self.function_static_arguments(2)?;
                        Node::Root(Box::new(args[0].clone()), Box::new(args[1].clone()))
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
                    NativeFunction::Pow => {
                        let args = self.function_static_arguments(2)?;
                        Node::Pow(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Log => {
                        let args = self.function_static_arguments(2)?;
                        Node::Log(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Mod => {
                        let args = self.function_static_arguments(2)?;
                        Node::Modulo(Box::new(args[0].clone()), Box::new(args[1].clone()))
                    }
                    NativeFunction::Min => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            return Err(ParseError::UnableToParse(
                                "There's no arguments in the min function".to_string(),
                            ));
                        }
                        Node::Min(Arc::new(args))
                    }
                    NativeFunction::Max => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            return Err(ParseError::UnableToParse(
                                "There's no arguments in the max function".to_string(),
                            ));
                        }
                        Node::Max(Arc::new(args))
                    }
                    NativeFunction::Avg => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            Node::Number(0)
                        } else {
                            Node::Avg(Arc::new(args))
                        }
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
            Token::LeftParen => self.get_enclosed_elements_with_impl_mult(
                OperPrec::DefaultZero,
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
            Token::Ampersand => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::And)?;
                Ok(Node::And(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Bar => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Or)?;
                Ok(Node::Or(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::LeftShift => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Shift)?;
                Ok(Node::LeftShift(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::RightShift => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Shift)?;
                Ok(Node::RightShift(Box::new(left_expr), Box::new(right_expr)))
            }
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
                Ok(Node::Pow(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::ExclamationMark => {
                self.get_next_token()?;
                self.implicit_multiply(Node::Factorial(Box::new(left_expr)))
            }
            Token::Superscript(script) => {
                self.get_next_token()?;
                Ok(Node::Pow(Box::new(left_expr), Box::new(Node::Number(script))))
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
    use crate::eval_i64::ast::Node::*;

    #[test]
    fn test_and() {
        let mut parser = Parser::new("1&2", None).unwrap();
        let expected = Node::And(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_or() {
        let mut parser = Parser::new("1|2", None).unwrap();
        let expected = Node::Or(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_left_shift() {
        let mut parser = Parser::new("1<<2", None).unwrap();
        let expected = Node::LeftShift(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_right_shift() {
        let mut parser = Parser::new("1>>2", None).unwrap();
        let expected = Node::RightShift(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_negative() {
        let mut parser = Parser::new("-1", None).unwrap();
        let expected = Negative(Box::new(Number(1)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_add() {
        let mut parser = Parser::new("1+2", None).unwrap();
        let expected = Add(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_subtract() {
        let mut parser = Parser::new("1-2", None).unwrap();
        let expected = Subtract(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_multiply() {
        let mut parser = Parser::new("1*2", None).unwrap();
        let expected = Multiply(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_divide() {
        let mut parser = Parser::new("1/2", None).unwrap();
        let expected = Divide(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo() {
        let mut parser = Parser::new("1%2", None).unwrap();
        let expected = Modulo(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_caret() {
        let mut parser = Parser::new("1^2", None).unwrap();
        let expected = Pow(Box::new(Number(1)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exclamation_mark() {
        let mut parser = Parser::new("1!", None).unwrap();
        let expected = Factorial(Box::new(Number(1)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo_function() {
        let mut parser = Parser::new("mod(3,2)", None).unwrap();
        let expected = Modulo(Box::new(Number(3)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_pow_function() {
        let mut parser = Parser::new("pow(3,2)", None).unwrap();
        let expected = Pow(Box::new(Number(3)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_log_function() {
        let mut parser = Parser::new("log(3,2)", None).unwrap();
        let expected = Log(Box::new(Number(3)), Box::new(Number(2)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_abs_function() {
        let mut parser = Parser::new("abs(5)", None).unwrap();
        let expected = Abs(Box::new(Number(5)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sqrt_function() {
        let mut parser = Parser::new("sqrt(55)", None).unwrap();
        let expected = Sqrt(Box::new(Number(55)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp_function() {
        let mut parser = Parser::new("exp(5)", None).unwrap();
        let expected = Exp(Box::new(Number(5)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_exp2_function() {
        let mut parser = Parser::new("exp2(5)", None).unwrap();
        let expected = Exp2(Box::new(Number(5)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ln_function() {
        let mut parser = Parser::new("ln(5)", None).unwrap();
        let expected = Ln(Box::new(Number(5)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sign_function() {
        let mut parser = Parser::new("sign(5)", None).unwrap();
        let expected = Sign(Box::new(Number(5)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function() {
        let mut parser = Parser::new("min(3)", None).unwrap();
        let expected = Min(Arc::new(vec![Number(3)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function2() {
        let mut parser = Parser::new("min(2,3,5)", None).unwrap();
        let expected = Min(Arc::new(vec![Number(2), Number(3), Number(5)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function() {
        let mut parser = Parser::new("max(3)", None).unwrap();
        let expected = Max(Arc::new(vec![Number(3)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function2() {
        let mut parser = Parser::new("max(2,3,5)", None).unwrap();
        let expected = Max(Arc::new(vec![Number(2), Number(3), Number(5)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_avg_function() {
        let mut parser = Parser::new("avg(3)", None).unwrap();
        let expected = Avg(Arc::new(vec![Number(3)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_avg_function2() {
        let mut parser = Parser::new("avg(2,3,5)", None).unwrap();
        let expected = Avg(Arc::new(vec![Number(2), Number(3), Number(5)]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_implicit_mul_prts() {
        let mut parser = Parser::new("(2)(3)", None).unwrap();
        let expected = Multiply(Box::new(Number(2)), Box::new(Number(3)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
