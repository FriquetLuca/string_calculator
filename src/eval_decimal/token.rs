use crate::utils::OperatorCategory;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq, Clone)]
pub enum NativeFunction {
    Abs,
    Truncate,
    Floor,
    Ceil,
    Round,
    Mod,
    Sign,
    Min,
    Max,
    Avg,
    Med,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
            ExplicitFunction(_) => OperatorCategory::Functional,
            _ => OperatorCategory::DefaultZero,
        }
    }
}
