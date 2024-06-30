use crate::utils::OperatorCategory;

#[derive(Debug, PartialEq, Clone)]
pub enum NativeFunction {
    Ln,
    Log,
    Pow,
    Sqrt,
    Root,
    Exp,
    Exp2,
    Abs,
    Mod,
    Sign,
    Min,
    Max,
    Avg,
    Med,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ampersand,
    Bar,
    LeftShift,
    RightShift,
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    ExclamationMark,
    Modulo,
    LeftParen,
    RightParen,
    Comma,
    ExplicitFunction(NativeFunction),
    Superscript(i64),
    Num(i64),
    Ans,
    Eof,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperatorCategory {
        use self::Token::*;
        match *self {
            Bar => OperatorCategory::BitwiseOr,
            Ampersand => OperatorCategory::BitwiseAnd,
            LeftShift | RightShift => OperatorCategory::Shift,
            Add | Subtract => OperatorCategory::Additive,
            Multiply | Divide | Modulo => OperatorCategory::Multiplicative,
            Caret | Superscript(_) => OperatorCategory::Power,
            ExclamationMark | ExplicitFunction(_) => OperatorCategory::Functional,
            _ => OperatorCategory::DefaultZero,
        }
    }
}
