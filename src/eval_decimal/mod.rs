mod ast;
mod parser;
mod token;
mod tokenizer;

use crate::utils::ParseError;
use ast::eval;
use parser::Parser;
use rust_decimal::Decimal;

/// Evaluate a formula inside a string and compute it into i64.
pub fn eval_decimal(expr: String, placeholder: Decimal) -> Result<Decimal, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(placeholder))?;
    let ast = math_parser.parse()?;
    Ok(eval(ast)?)
}
