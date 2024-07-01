use std::{error, sync::Arc};

use rust_decimal::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Factorial(Box<Node>),
    Abs(Box<Node>),
    Sign(Box<Node>),
    Min(Arc<Vec<Node>>),
    Max(Arc<Vec<Node>>),
    Avg(Arc<Vec<Node>>),
    Med(Arc<Vec<Node>>),
    Floor(Box<Node>),
    Ceil(Box<Node>),
    Round(Box<Node>),
    Truncate(Box<Node>),
    Sqrt(Box<Node>),
    Root(Box<Node>, Box<Node>),
    ILog(Box<Node>, Box<Node>),
    LambertW(Box<Node>),
    Ln(Box<Node>),
    Lb(Box<Node>),
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
    Number(Decimal),
}

fn gamma(a: Decimal) -> Decimal {
    let mut s = Decimal::new(2485740891387535655, 27);
    if a < Decimal::new(5, 1) {
        s += Decimal::new(1051423785817219742, 20) / (Decimal::new(1, 0) - a);
        s += Decimal::new(-3456870972220162354, 22) / (Decimal::new(2, 0) - a);
        s += Decimal::new(4512277094668948237, 20) / (Decimal::new(3, 0) - a);
        s += Decimal::new(-2982852253235766557, 22) / (Decimal::new(4, 0) - a);
        s += Decimal::new(1056397115771267131, 22) / (Decimal::new(5, 0) - a);
        s += Decimal::new(-1954287731916458696, 23) / (Decimal::new(6, 0) - a);
        s += Decimal::new(1709705434044412243, 24) / (Decimal::new(7, 0) - a);
        s += Decimal::new(-5719261174043057813, 24) / (Decimal::new(8, 0) - a);
        s += Decimal::new(4633994733599056367, 28) / (Decimal::new(9, 0) - a);
        s += Decimal::new(-2719949084886077039, 31) / (Decimal::new(10, 0) - a);
        let compute_sin = (Decimal::new(3141592653589793238, 18) * a).sin(); // 3.14159265358979323846264338327950288419716939937510582
        let compute_pow = ((a - Decimal::new(10400511, 6)) / Decimal::new(2718281828459045235, 18))
            .powd(Decimal::new(5, 1) - a);
        Decimal::new(3141592653589793238, 18)
            / (compute_sin * s * Decimal::new(1860382734205265717, 18) * compute_pow)
    } else {
        s += Decimal::new(1051423785817219742, 20) / a;
        s += Decimal::new(-3456870972220162354, 22) / (a + Decimal::new(1, 0));
        s += Decimal::new(4512277094668948237, 20) / (a + Decimal::new(2, 0));
        s += Decimal::new(-2982852253235766557, 22) / (a + Decimal::new(3, 0));
        s += Decimal::new(1056397115771267131, 22) / (a + Decimal::new(4, 0));
        s += Decimal::new(-1954287731916458696, 23) / (a + Decimal::new(5, 0));
        s += Decimal::new(1709705434044412243, 24) / (a + Decimal::new(6, 0));
        s += Decimal::new(-5719261174043057813, 24) / (a + Decimal::new(7, 0));
        s += Decimal::new(4633994733599056367, 28) / (a + Decimal::new(8, 0));
        s += Decimal::new(-2719949084886077039, 31) / (a + Decimal::new(9, 0));
        let compute_pow = ((a + Decimal::new(10400511, 6)) / Decimal::new(2718281828459045235, 18))
            .powd(a - Decimal::new(5, 1));
        s * Decimal::new(1860382734205265717, 18) * compute_pow
    }
}

pub fn eval(expr: Node) -> Result<Decimal, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Modulo(expr1, expr2) => Ok(eval(*expr1)? % eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Abs(sub_expr) => Ok(eval(*sub_expr)?.abs()),
        Floor(sub_expr) => Ok(eval(*sub_expr)?.floor()),
        Ceil(sub_expr) => Ok(eval(*sub_expr)?.ceil()),
        Round(sub_expr) => Ok(eval(*sub_expr)?.round()),
        Truncate(sub_expr) => Ok(eval(*sub_expr)?.trunc()),
        Sign(sub_expr) => Ok(eval(*sub_expr)?.signum()),
        Ln(sub_expr) => Ok(eval(*sub_expr)?.ln()),
        Lb(sub_expr) => Ok(eval(*sub_expr)?.ln() / Decimal::new(2, 0).ln()),
        Exp(sub_expr) => Ok(eval(*sub_expr)?.exp()),
        Exp2(sub_expr) => Ok(Decimal::new(2, 0).powd(eval(*sub_expr)?)),
        Pow(expr1, expr2) => Ok(eval(*expr1)?.powd(eval(*expr2)?)),
        Log(expr1, expr2) => Ok(eval(*expr1)?.ln() / eval(*expr2)?.ln()),
        Factorial(sub_expr) => {
            let sub_result = eval(*sub_expr)?;
            if sub_result >= Decimal::ZERO {
                if (sub_result % Decimal::new(1, 0)) > Decimal::ZERO {
                    Ok(gamma(sub_result + Decimal::new(1, 0)))
                } else {
                    let mut factorial_result = Decimal::new(1, 0);
                    for i in 2..=sub_result.to_i64().unwrap() {
                        factorial_result *= Decimal::new(i, 0);
                    }
                    Ok(factorial_result)
                }
            } else if (sub_result % Decimal::new(1, 0)) == Decimal::ZERO {
                return Err("The factorial function is not defined for {}.".into());
            } else {
                Ok(gamma(sub_result + Decimal::new(1, 0)))
            }
        }
        LambertW(expr) => {
            let sub_expr = eval(*expr)?;
            if sub_expr < -Decimal::new(-1, 0).exp() {
                return Err("The Lambert W function is not defined for {}.".into());
            }
            let iterations = (Decimal::new(4, 0))
                .max((sub_expr.log10() / Decimal::new(3, 0)).ceil())
                .to_i32()
                .unwrap_or(4);
            let mut w = Decimal::ZERO;
            for _ in 0..iterations {
                let exp_w = w.exp();
                w -= (w * exp_w - sub_expr)
                    / (exp_w * (w + Decimal::new(1, 0))
                        - (w + Decimal::new(2, 0)) * (w * exp_w - sub_expr)
                            / (Decimal::new(2, 0) * w + Decimal::new(2, 0)));
            }
            Ok(w)
        }
        ILog(expr1, expr2) => {
            let mut n = eval(*expr1)?;
            let b = eval(*expr2)?;
            let mut x = Decimal::ZERO;
            while n > Decimal::new(1, 0) {
                x += Decimal::new(1, 0);
                n = (n.log10() / b.log10()).floor();
            }
            Ok(x)
        }
        Sqrt(sub_expr) => match eval(*sub_expr)?.sqrt() {
            Some(result) => Ok(result),
            None => Err("Unable to compute the square root of negative number".into()),
        },
        Root(n_th_expr, x_expr) => Ok(eval(*x_expr)?.powd(Decimal::new(1, 0) / eval(*n_th_expr)?)),
        Min(args) => {
            if args.len() > 1 {
                let mut result = Decimal::MAX;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().min(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(Decimal::ZERO),
                }
            }
        }
        Max(args) => {
            if args.len() > 1 {
                let mut result = Decimal::MIN;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().max(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(Decimal::ZERO),
                }
            }
        }
        Avg(args) => {
            let mut result = Decimal::ZERO;
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                result += eval(arg).unwrap();
            }
            Ok(result / Decimal::new(args.len() as i64, 0))
        }
        Med(args) => {
            let mut results = vec![];
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                results.push(eval(arg).unwrap());
            }
            results.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = results.len();
            if len % 2 == 0 {
                Ok((results[len >> 1] + results[(len >> 1) - 1]) / Decimal::new(2, 0))
            } else {
                Ok(results[len >> 1])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_decimal::{ast::eval, parser::Parser};
    use rust_decimal::Decimal;

    #[test]
    fn test_expr1() {
        let ast = Parser::new("1+2-3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Decimal::ZERO);
    }
    #[test]
    fn test_expr2() {
        let ast = Parser::new("3+2-1*5/4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Decimal::new(375, 2));
    }
    #[test]
    fn test_expr3() {
        let ast = Parser::new("2*4%3/2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Decimal::new(1, 0));
    }
    #[test]
    fn test_expr4() {
        let ast = Parser::new("med(5,2,8,9,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Decimal::new(7, 0));
    }
    #[test]
    fn test_expr5() {
        let ast = Parser::new("med(5,2,8,9)", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Decimal::new(65, 1));
    }
}
