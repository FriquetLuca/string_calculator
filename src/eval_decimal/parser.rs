use std::sync::Arc;

use rust_decimal::Decimal;

use super::ast::Node;
use super::token::{NativeFunction, Token};
use super::tokenizer::Tokenizer;
use crate::utils::{OperatorCategory, ParseError};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
    previous_token: Option<Token>,
    placeholder: Decimal,
}
impl<'a> Parser<'a> {
    pub fn new(expr: &'a str, placeholder: Option<Decimal>) -> Result<Self, ParseError> {
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
    fn function_arguments(&mut self) -> Result<Vec<Node>, ParseError> {
        self.find_item_list(
            Token::LeftParen,
            Token::RightParen,
            OperatorCategory::DefaultZero,
        )
    }
    fn find_item_list(
        &mut self,
        start_token: Token,
        end_token: Token,
        oper_prec: OperatorCategory,
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
                    NativeFunction::Sign => {
                        Node::Sign(Box::new(self.function_static_arguments(1)?[0].clone()))
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
                            Node::Number(Decimal::new(0, 0))
                        } else {
                            Node::Avg(Arc::new(args))
                        }
                    }
                    NativeFunction::Med => {
                        let args = self.function_arguments()?;
                        if args.is_empty() {
                            return Err(ParseError::UnableToParse(
                                "Cannot compute the median of no arguments".to_string(),
                            ));
                        } else {
                            Node::Med(Arc::new(args))
                        }
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
                    NativeFunction::Truncate => {
                        Node::Truncate(Box::new(self.function_static_arguments(1)?[0].clone()))
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
            Token::LeftParen => self.get_enclosed_elements_with_impl_mult(
                OperatorCategory::DefaultZero,
                Token::RightParen,
                |expr| expr,
            ),
            Token::LeftFloor => self.get_enclosed_elements_with_impl_mult(
                OperatorCategory::DefaultZero,
                Token::RightFloor,
                |expr| Node::Floor(Box::new(expr)),
            ),
            Token::LeftCeiling => self.get_enclosed_elements_with_impl_mult(
                OperatorCategory::DefaultZero,
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
            Token::Modulo => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorCategory::Multiplicative)?;
                Ok(Node::Modulo(Box::new(left_expr), Box::new(right_expr)))
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
    use crate::eval_decimal::ast::Node::*;

    #[test]
    fn test_floor() {
        let mut parser = Parser::new("⌊5.25⌋", None).unwrap();
        let expected = Floor(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ceil() {
        let mut parser = Parser::new("⌈5.25⌉", None).unwrap();
        let expected = Ceil(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_floor_function() {
        let mut parser = Parser::new("floor(5.25)", None).unwrap();
        let expected = Floor(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_ceil_function() {
        let mut parser = Parser::new("ceil(5.25)", None).unwrap();
        let expected = Ceil(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_round_function() {
        let mut parser = Parser::new("round(5.25)", None).unwrap();
        let expected = Round(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_truncate_function() {
        let mut parser = Parser::new("truncate(5.25)", None).unwrap();
        let expected = Truncate(Box::new(Number(Decimal::new(525, 2))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_negative() {
        let mut parser = Parser::new("-1", None).unwrap();
        let expected = Negative(Box::new(Number(Decimal::new(1, 0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_add() {
        let mut parser = Parser::new("1+2", None).unwrap();
        let expected = Add(
            Box::new(Number(Decimal::new(1, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_subtract() {
        let mut parser = Parser::new("1-2", None).unwrap();
        let expected = Subtract(
            Box::new(Number(Decimal::new(1, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_multiply() {
        let mut parser = Parser::new("1*2", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Decimal::new(1, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_divide() {
        let mut parser = Parser::new("1/2", None).unwrap();
        let expected = Divide(
            Box::new(Number(Decimal::new(1, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo() {
        let mut parser = Parser::new("1%2", None).unwrap();
        let expected = Modulo(
            Box::new(Number(Decimal::new(1, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_modulo_function() {
        let mut parser = Parser::new("mod(3,2)", None).unwrap();
        let expected = Modulo(
            Box::new(Number(Decimal::new(3, 0))),
            Box::new(Number(Decimal::new(2, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_abs_function() {
        let mut parser = Parser::new("abs(5)", None).unwrap();
        let expected = Abs(Box::new(Number(Decimal::new(5, 0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_sign_function() {
        let mut parser = Parser::new("sign(5)", None).unwrap();
        let expected = Sign(Box::new(Number(Decimal::new(5, 0))));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function() {
        let mut parser = Parser::new("min(3)", None).unwrap();
        let expected = Min(Arc::new(vec![Number(Decimal::new(3, 0))]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_min_function2() {
        let mut parser = Parser::new("min(2,3,5)", None).unwrap();
        let expected = Min(Arc::new(vec![
            Number(Decimal::new(2, 0)),
            Number(Decimal::new(3, 0)),
            Number(Decimal::new(5, 0)),
        ]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function() {
        let mut parser = Parser::new("max(3)", None).unwrap();
        let expected = Max(Arc::new(vec![Number(Decimal::new(3, 0))]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_max_function2() {
        let mut parser = Parser::new("max(2,3,5)", None).unwrap();
        let expected = Max(Arc::new(vec![
            Number(Decimal::new(2, 0)),
            Number(Decimal::new(3, 0)),
            Number(Decimal::new(5, 0)),
        ]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_avg_function() {
        let mut parser = Parser::new("avg(3)", None).unwrap();
        let expected = Avg(Arc::new(vec![Number(Decimal::new(3, 0))]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_avg_function2() {
        let mut parser = Parser::new("avg(2,3,5)", None).unwrap();
        let expected = Avg(Arc::new(vec![
            Number(Decimal::new(2, 0)),
            Number(Decimal::new(3, 0)),
            Number(Decimal::new(5, 0)),
        ]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_implicit_mul_prts() {
        let mut parser = Parser::new("(2)(3)", None).unwrap();
        let expected = Multiply(
            Box::new(Number(Decimal::new(2, 0))),
            Box::new(Number(Decimal::new(3, 0))),
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_med_function() {
        let mut parser = Parser::new("med(3)", None).unwrap();
        let expected = Med(Arc::new(vec![Number(Decimal::new(3, 0))]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
    #[test]
    fn test_med_function2() {
        let mut parser = Parser::new("med(2,3,5)", None).unwrap();
        let expected = Med(Arc::new(vec![
            Number(Decimal::new(2, 0)),
            Number(Decimal::new(3, 0)),
            Number(Decimal::new(5, 0)),
        ]));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}