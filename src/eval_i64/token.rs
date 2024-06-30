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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Pow2,
    Pow3,
    Comma,
    ExplicitFunction(NativeFunction),
    Num(i64),
    Ans,
    Eof,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperPrec {
    DefaultZero,
    Shift,
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
            LeftShift | RightShift => Shift,
            Add | Subtract => AddSub,
            Multiply | Divide | Modulo => MulDiv,
            Caret | Pow2 | Pow3 => Power,
            ExclamationMark | ExplicitFunction(_) => Functional,
            _ => DefaultZero,
        }
    }
}
