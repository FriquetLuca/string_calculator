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
    Log,
    Pow,
    Sqrt,
    Root,
    Exp,
    Exp2,
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
    Superscript(f64),
    Num(f64),
    Ans,
    Eof,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
    Functional,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide | Modulo | DegToRad | RadToDeg => MulDiv,
            Caret | Superscript(_) => Power,
            ExclamationMark | ExplicitFunction(_) => Functional,
            _ => DefaultZero,
        }
    }
}
