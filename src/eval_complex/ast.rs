use num_complex::Complex;
use std::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Root(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Abs(Box<Node>),
    Sin(Box<Node>),
    Cos(Box<Node>),
    Tan(Box<Node>),
    Sinh(Box<Node>),
    Cosh(Box<Node>),
    Tanh(Box<Node>),
    Arsinh(Box<Node>),
    Arcosh(Box<Node>),
    Artanh(Box<Node>),
    Asin(Box<Node>),
    Acos(Box<Node>),
    Atan(Box<Node>),
    Sqrt(Box<Node>),
    Ln(Box<Node>),
    Lb(Box<Node>),
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Number(Complex<f64>),
}

pub fn eval(expr: Node) -> Result<Complex<f64>, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Pow(expr1, expr2) => Ok(eval(*expr1)?.powc(eval(*expr2)?)),
        Root(n_th_expr, x_expr) => Ok(eval(*x_expr)?.powc(1.0 / eval(*n_th_expr)?)),
        Abs(sub_expr) => Ok(Complex::new(eval(*sub_expr)?.norm(), 0.0)),
        Sin(sub_expr) => Ok(eval(*sub_expr)?.sin()),
        Cos(sub_expr) => Ok(eval(*sub_expr)?.cos()),
        Tan(sub_expr) => Ok(eval(*sub_expr)?.tan()),
        Sinh(sub_expr) => Ok(eval(*sub_expr)?.sinh()),
        Cosh(sub_expr) => Ok(eval(*sub_expr)?.cosh()),
        Tanh(sub_expr) => Ok(eval(*sub_expr)?.tanh()),
        Asin(sub_expr) => Ok(eval(*sub_expr)?.asin()),
        Acos(sub_expr) => Ok(eval(*sub_expr)?.acos()),
        Atan(sub_expr) => Ok(eval(*sub_expr)?.atan()),
        Arsinh(sub_expr) => Ok(eval(*sub_expr)?.asinh()),
        Arcosh(sub_expr) => Ok(eval(*sub_expr)?.acosh()),
        Artanh(sub_expr) => Ok(eval(*sub_expr)?.atanh()),
        Sqrt(sub_expr) => Ok(eval(*sub_expr)?.sqrt()),
        Ln(sub_expr) => Ok(eval(*sub_expr)?.ln()),
        Lb(sub_expr) => Ok(eval(*sub_expr)?.log(2.0)),
        Exp(sub_expr) => Ok(eval(*sub_expr)?.exp()),
        Exp2(sub_expr) => Ok(eval(*sub_expr)?.exp2()),
        Log(expr1, expr2) => Ok(eval(*expr1)?.ln() / eval(*expr2)?.ln()),
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use num_complex::Complex;

    use crate::eval_complex::{ast::eval, parser::Parser};

    #[test]
    fn test_expr1() {
        let ast = Parser::new("1+2-3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(0.0, 0.0));
    }
    #[test]
    fn test_expr2() {
        let ast = Parser::new("3+2-1*5/4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(3.75, 0.0));
    }
    #[test]
    fn test_expr3() {
        let ast = Parser::new("5+(2*7)*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(47.0, 0.0));
    }
    #[test]
    fn test_expr4() {
        let ast = Parser::new("3*2^3*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(71.99999999999997, 0.0));
    }
    #[test]
    fn test_expr5() {
        let ast = Parser::new("-i", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(0.0, -1.0));
    }
    #[test]
    fn test_expr6() {
        let ast = Parser::new("i²", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Complex::new(-1.0, 1.2246467991473532e-16));
    }
}
