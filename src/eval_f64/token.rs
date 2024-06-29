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
    Pow2,
    Pow3,
    E,
    Pi,
    Comma,
    Bar,
    DegToRad,
    RadToDeg,
    ExplicitFunction(NativeFunction),
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
            Caret | Pow2 | Pow3 => Power,
            ExclamationMark | ExplicitFunction(_) => Functional,
            _ => DefaultZero,
        }
    }
}
