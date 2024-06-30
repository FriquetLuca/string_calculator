use conv::ValueFrom;
use statrs::function::gamma::gamma;
use std::{error, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Atan2(Box<Node>, Box<Node>),
    Root(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Factorial(Box<Node>),
    Abs(Box<Node>),
    Floor(Box<Node>),
    Ceil(Box<Node>),
    Round(Box<Node>),
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
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Truncate(Box<Node>),
    Sign(Box<Node>),
    Min(Arc<Vec<Node>>),
    Max(Arc<Vec<Node>>),
    Avg(Arc<Vec<Node>>),
    Med(Arc<Vec<Node>>),
    Number(f64),
}

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Modulo(expr1, expr2) => Ok(eval(*expr1)? % eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Pow(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Root(n_th_expr, x_expr) => Ok(eval(*x_expr)?.powf(1.0 / eval(*n_th_expr)?)),
        Factorial(sub_expr) => {
            let sub_result = eval(*sub_expr)?;
            if sub_result >= 0.0 {
                if (sub_result % 1.0) > 0.0 {
                    Ok(gamma(sub_result + 1.0))
                } else {
                    let mut factorial_result = 1.0;
                    for i in 2..=(sub_result as usize) {
                        factorial_result *= i as f64;
                    }
                    Ok(factorial_result)
                }
            } else if (sub_result % 1.0) == 0.0 {
                Ok(f64::NAN)
            } else {
                Ok(gamma(sub_result + 1.0))
            }
        }
        Abs(sub_expr) => Ok(eval(*sub_expr)?.abs()),
        Floor(sub_expr) => Ok(eval(*sub_expr)?.floor()),
        Ceil(sub_expr) => Ok(eval(*sub_expr)?.ceil()),
        Round(sub_expr) => Ok(eval(*sub_expr)?.round()),
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
        Truncate(sub_expr) => Ok(eval(*sub_expr)?.trunc()),
        Sign(sub_expr) => Ok(eval(*sub_expr)?.signum()),
        Exp(sub_expr) => Ok(eval(*sub_expr)?.exp()),
        Exp2(sub_expr) => Ok(eval(*sub_expr)?.exp2()),
        Log(expr1, expr2) => Ok(eval(*expr1)?.log(eval(*expr2)?)),
        Min(args) => {
            if args.len() > 1 {
                let mut result = f64::INFINITY;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().min(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0.0),
                }
            }
        }
        Max(args) => {
            if args.len() > 1 {
                let mut result = f64::NEG_INFINITY;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().max(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0.0),
                }
            }
        }
        Avg(args) => {
            let mut result = 0.0;
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                result += eval(arg).unwrap();
            }
            Ok(result / f64::value_from(args.len())?)
        }
        Med(args) => {
            let mut results = vec![];
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                results.push(eval(arg).unwrap());
            }
            results.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = results.len();
            if len % 2 == 0 {
                Ok((results[len >> 1] + results[(len >> 1) - 1]) / 2.0)
            } else {
                Ok(results[len >> 1])
            }
        }
        Atan2(expr1, expr2) => Ok(eval(*expr1)?.atan2(eval(*expr2)?)),
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use crate::eval_f64::{ast::eval, parser::Parser};

    #[test]
    fn test_expr1() {
        let ast = Parser::new("1+2-3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        let ast = Parser::new("3+2-1*5/4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
    #[test]
    fn test_expr3() {
        let ast = Parser::new("5+(2*7-3!)*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 29.0);
    }
    #[test]
    fn test_expr4() {
        let ast = Parser::new("2*4%3/2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 1.0);
    }
    #[test]
    fn test_expr5() {
        let ast = Parser::new("3*2^3*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 72.0);
    }
    #[test]
    fn test_expr6() {
        let ast = Parser::new("2+3*atan2(3,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 2.0 + 3.0 * (3.0 as f64).atan2(7.0));
    }
    #[test]
    fn test_expr7() {
        let ast = Parser::new("med(5,2,8,9,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 7.0);
    }
    #[test]
    fn test_expr8() {
        let ast = Parser::new("med(5,2,8,9)", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 6.5);
    }
}
