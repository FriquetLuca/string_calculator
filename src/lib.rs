#[cfg(feature = "eval_decimal")]
mod eval_decimal;
#[cfg(feature = "eval_f64")]
mod eval_f64;
#[cfg(feature = "eval_i64")]
mod eval_i64;
#[cfg(any(feature = "eval_decimal", feature = "eval_f64", feature = "eval_i64"))]
mod utils;

#[cfg(feature = "eval_decimal")]
pub use eval_decimal::eval_decimal;
#[cfg(feature = "eval_f64")]
pub use eval_f64::eval_f64;
#[cfg(feature = "eval_i64")]
pub use eval_i64::eval_i64;
#[cfg(any(feature = "eval_decimal", feature = "eval_f64", feature = "eval_i64"))]
pub use utils::ParseError;
