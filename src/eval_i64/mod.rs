mod ast;
mod parser;
mod token;
mod tokenizer;

use crate::utils::ParseError;
use ast::eval;
use parser::Parser;

/// Evaluate a formula inside a string and compute it into i64.
pub fn eval_i64(expr: String, placeholder: i64) -> Result<i64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(placeholder))?;
    let ast = math_parser.parse()?;
    Ok(eval(ast)?)
}
