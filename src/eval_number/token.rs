use crate::utils::OperatorCategory;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Float(f64),
    Integer(i64),
}

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
    Atan2,
    Arcosh,
    Arsinh,
    Artanh,
    Ln,
    Lb,
    Log,
    ILog,
    Pow,
    Sqrt,
    Root,
    Exp,
    Exp2,
    LambertW,
    Abs,
    Sign,
    Truncate,
    Floor,
    Ceil,
    Round,
    Min,
    Max,
    Avg,
    Med,
    Mod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    ExclamationMark,
    Modulo,
    LeftParen,
    RightParen,
    LeftFloor,
    RightFloor,
    LeftCeiling,
    RightCeiling,
    E,
    Pi,
    Comma,
    DegToRad,
    RadToDeg,
    ExplicitFunction(NativeFunction),
    Superscript(Number),
    Num(Number),
    Ans,
    Eof,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperatorCategory {
        use self::Token::*;
        match *self {
            Add | Subtract => OperatorCategory::Additive,
            Multiply | Divide | Modulo | DegToRad | RadToDeg => OperatorCategory::Multiplicative,
            Caret | Superscript(_) => OperatorCategory::Power,
            ExclamationMark | ExplicitFunction(_) => OperatorCategory::Functional,
            _ => OperatorCategory::DefaultZero,
        }
    }
}
