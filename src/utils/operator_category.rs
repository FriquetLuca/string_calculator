#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OperatorCategory {
    DefaultZero,
    BitwiseOr,
    BitwiseAnd,
    Shift,
    Additive,
    Multiplicative,
    Power,
    Negative,
    Functional,
}
