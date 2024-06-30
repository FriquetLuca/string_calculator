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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperPrec {
    DefaultZero,
    Or,
    And,
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
            Bar => Or,
            Ampersand => And,
            LeftShift | RightShift => Shift,
            Add | Subtract => AddSub,
            Multiply | Divide | Modulo => MulDiv,
            Caret | Superscript(_) => Power,
            ExclamationMark | ExplicitFunction(_) => Functional,
            _ => DefaultZero,
        }
    }
}
