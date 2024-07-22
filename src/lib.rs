#[cfg(feature = "eval_complex")]
mod eval_complex;
#[cfg(feature = "eval_decimal")]
mod eval_decimal;
#[cfg(feature = "eval_f64")]
mod eval_f64;
#[cfg(feature = "eval_i64")]
mod eval_i64;
#[cfg(feature = "eval_number")]
mod eval_number;
#[cfg(any(
    feature = "eval_decimal",
    feature = "eval_f64",
    feature = "eval_i64",
    feature = "eval_complex",
    feature = "eval_number"
))]
mod utils;

#[cfg(feature = "eval_complex")]
pub use eval_complex::eval_complex;
#[cfg(feature = "eval_decimal")]
pub use eval_decimal::eval_decimal;
#[cfg(feature = "eval_f64")]
pub use eval_f64::eval_f64;
#[cfg(feature = "eval_i64")]
pub use eval_i64::eval_i64;
#[cfg(feature = "eval_number")]
pub use eval_number::{eval_number, Number};
#[cfg(any(
    feature = "eval_decimal",
    feature = "eval_f64",
    feature = "eval_i64",
    feature = "eval_complex",
    feature = "eval_number"
))]
pub use utils::ParseError;
