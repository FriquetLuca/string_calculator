mod ast;
mod parser;
mod token;
mod tokenizer;

use ast::eval;
use parser::Parser;
use crate::utils::ParseError;

/// Evaluate a formula inside a string and compute it into f64.
pub fn eval_f64(expr: String, placeholder: f64) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(placeholder))?;
    let ast = math_parser.parse()?;
    Ok(eval(ast)?)
}
