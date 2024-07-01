use crate::utils::OperatorCategory;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Clone)]
pub enum NativeFunction {
    Abs,
    Truncate,
    Floor,
    Ceil,
    Round,
    Ln,
    Log,
    Pow,
    Sqrt,
    Root,
    Exp,
    Exp2,
    Mod,
    Sign,
    Min,
    Max,
    Avg,
    Med,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    E,
    Pi,
    Caret,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftParen,
    RightParen,
    Comma,
    LeftFloor,
    RightFloor,
    LeftCeiling,
    RightCeiling,
    ExplicitFunction(NativeFunction),
    Superscript(Decimal),
    Num(Decimal),
    Ans,
    Eof,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperatorCategory {
        use self::Token::*;
        match *self {
            Add | Subtract => OperatorCategory::Additive,
            Multiply | Divide | Modulo => OperatorCategory::Multiplicative,
            Caret | Superscript(_) => OperatorCategory::Power,
            ExplicitFunction(_) => OperatorCategory::Functional,
            _ => OperatorCategory::DefaultZero,
        }
    }
}
