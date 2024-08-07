mod ast;
mod parser;
mod token;
mod tokenizer;

use crate::utils::ParseError;
use ast::eval;
use num_complex::Complex;
use parser::Parser;

/// Evaluate a formula inside a string and compute it into f64.
pub fn eval_complex(expr: String, placeholder: Complex<f64>) -> Result<Complex<f64>, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr, Some(placeholder))?;
    let ast = math_parser.parse()?;
    Ok(eval(ast)?)
}
