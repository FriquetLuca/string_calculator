mod ast;
mod number;
mod parser;
mod token;
mod tokenizer;

use crate::utils::ParseError;
use ast::eval;
pub use number::Number;
use parser::Parser;

/// Evaluate a formula inside a string and compute it into f64.
pub fn eval_number(expr: String, placeholder: Number) -> Result<Number, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(placeholder))?;
    let ast = math_parser.parse()?;
    let result = eval(ast)?;
    Ok(result)
}
