use num_complex::Complex;

use crate::utils::OperatorCategory;

#[derive(Debug, PartialEq, Clone)]
pub enum NativeFunction {
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
    Asin,
    Acos,
    Atan,
    Arcosh,
    Arsinh,
    Artanh,
    Ln,
    Lb,
    Log,
    Pow,
    Sqrt,
    Root,
    Exp,
    Exp2,
    Abs,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    E,
    Pi,
    Comma,
    DegToRad,
    RadToDeg,
    ExplicitFunction(NativeFunction),
    Superscript(Complex<f64>),
    Num(Complex<f64>),
    Ans,
    Eof,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperatorCategory {
        use self::Token::*;
        match *self {
            Add | Subtract => OperatorCategory::Additive,
            Multiply | Divide | DegToRad | RadToDeg => OperatorCategory::Multiplicative,
            Caret | Superscript(_) => OperatorCategory::Power,
            ExplicitFunction(_) => OperatorCategory::Functional,
            _ => OperatorCategory::DefaultZero,
        }
    }
}
