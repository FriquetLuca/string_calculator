#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperatorCategory {
    DefaultZero,
    #[cfg(feature = "eval_i64")]
    BitwiseOr,
    #[cfg(feature = "eval_i64")]
    BitwiseAnd,
    #[cfg(feature = "eval_i64")]
    Shift,
    Additive,
    Multiplicative,
    Power,
    Negative,
    Functional,
}
