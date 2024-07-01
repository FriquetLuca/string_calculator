use std::{error, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    LeftShift(Box<Node>, Box<Node>),
    RightShift(Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Root(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Factorial(Box<Node>),
    Abs(Box<Node>),
    Sqrt(Box<Node>),
    Ln(Box<Node>),
    Lb(Box<Node>),
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Sign(Box<Node>),
    Min(Arc<Vec<Node>>),
    Max(Arc<Vec<Node>>),
    Avg(Arc<Vec<Node>>),
    Med(Arc<Vec<Node>>),
    Gcd(Arc<Vec<Node>>),
    Lcm(Arc<Vec<Node>>),
    Number(i64),
}

fn gcd(expr1: i64, expr2: i64) -> i64 {
    let mut a = expr1;
    let mut b = expr2;
    while b != 0 {
        let remainder = a % b;
        a = expr2;
        b = remainder;
    }
    a.abs()
}

fn lcm(expr1: i64, expr2: i64) -> i64 {
    if expr1 == 0 || expr2 == 0 {
        return 0;
    }
    (expr1 / gcd(expr1, expr2) * expr2).abs()
}

pub fn eval(expr: Node) -> Result<i64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        And(expr1, expr2) => Ok(eval(*expr1)? & eval(*expr2)?),
        Or(expr1, expr2) => Ok(eval(*expr1)? | eval(*expr2)?),
        LeftShift(expr1, expr2) => Ok(eval(*expr1)? << eval(*expr2)?),
        RightShift(expr1, expr2) => Ok(eval(*expr1)? >> eval(*expr2)?),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Modulo(expr1, expr2) => Ok(eval(*expr1)? % eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Pow(expr1, expr2) => Ok(eval(*expr1)?.pow(eval(*expr2)? as u32)),
        Factorial(sub_expr) => {
            let sub_result = eval(*sub_expr)?;
            if sub_result >= 0 {
                let mut factorial_result = 1;
                for i in 2..=(sub_result as usize) {
                    factorial_result *= i as i64;
                }
                Ok(factorial_result)
            } else {
                Ok(0)
            }
        }
        Abs(sub_expr) => Ok(eval(*sub_expr)?.abs()),
        Sqrt(sub_expr) => {
            let before_sqr = eval(*sub_expr)? as f64;
            Ok(before_sqr.sqrt() as i64)
        }
        Root(n_th_expr, x_expr) => {
            let n_th_expr = eval(*n_th_expr)? as f64;
            let x_expr = eval(*x_expr)? as f64;
            Ok(x_expr.powf(1.0 / n_th_expr) as i64)
        }
        Ln(sub_expr) => {
            let before_sqr = eval(*sub_expr)? as f64;
            Ok(before_sqr.ln() as i64)
        }
        Lb(sub_expr) => {
            let before_sqr = eval(*sub_expr)? as f64;
            Ok(before_sqr.log(2.0) as i64)
        }
        Sign(sub_expr) => Ok(eval(*sub_expr)?.signum()),
        Exp(sub_expr) => Ok((eval(*sub_expr)? as f64).exp() as i64),
        Exp2(sub_expr) => {
            let result = eval(*sub_expr)?;
            if result < 0 {
                Ok(0)
            } else {
                Ok(1 << result)
            }
        }
        Log(expr1, expr2) => {
            let eval_1 = eval(*expr1)? as f64;
            let eval_2 = eval(*expr2)? as f64;
            Ok(eval_1.log(eval_2) as i64)
        }
        Gcd(args) => {
            // Ok(gcd(eval(*expr1)?, eval(*expr2)?))
            if args.len() > 1 {
                let mut result: Option<i64> = None;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    let right_art = eval(arg)?;
                    result = result
                        .map(|left_arg| Some(gcd(left_arg, right_art)))
                        .unwrap_or(Some(right_art));
                }
                Ok(result.unwrap())
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0),
                }
            }
        }
        Lcm(args) => {
            if args.len() > 1 {
                let mut result: Option<i64> = None;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    let right_art = eval(arg)?;
                    result = result
                        .map(|left_arg| Some(lcm(left_arg, right_art)))
                        .unwrap_or(Some(right_art));
                }
                Ok(result.unwrap())
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0),
                }
            }
        }
        Min(args) => {
            if args.len() > 1 {
                let mut result = i64::MIN;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().min(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0),
                }
            }
        }
        Max(args) => {
            if args.len() > 1 {
                let mut result = i64::MAX;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    result = eval(arg).unwrap().max(result);
                }
                Ok(result)
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(0),
                }
            }
        }
        Avg(args) => {
            let mut result = 0;
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                result += eval(arg).unwrap();
            }
            Ok(result / (args.len() as i64))
        }
        Med(args) => {
            let mut results = vec![];
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                results.push(eval(arg).unwrap());
            }
            results.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let len = results.len();
            if len % 2 == 0 {
                Ok((results[len >> 1] + results[(len >> 1) - 1]) / 2)
            } else {
                Ok(results[len >> 1])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eval_i64::{ast::eval, parser::Parser};

    #[test]
    fn test_expr1() {
        let ast = Parser::new("1+2-3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0);
    }
    #[test]
    fn test_expr2() {
        let ast = Parser::new("3+2-1*5/4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 4);
    }
    #[test]
    fn test_expr3() {
        let ast = Parser::new("5+(2*7-3!)*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 29);
    }
    #[test]
    fn test_expr4() {
        let ast = Parser::new("2*4%3/2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 1);
    }
    #[test]
    fn test_expr5() {
        let ast = Parser::new("3*2^3*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 72);
    }
    #[test]
    fn test_expr6() {
        let ast = Parser::new("1<<2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 4);
    }
    #[test]
    fn test_expr7() {
        let ast = Parser::new("root(2,35)", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 5);
    }
    #[test]
    fn test_expr8() {
        let ast = Parser::new("1|2|4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 7);
    }
    #[test]
    fn test_expr9() {
        let ast = Parser::new("1&3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 1);
    }
    #[test]
    fn test_expr10() {
        let ast = Parser::new("1+1&3|1", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3);
    }
    #[test]
    fn test_expr11() {
        let ast = Parser::new("med(5,2,8,9,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 7);
    }
    #[test]
    fn test_expr12() {
        let ast = Parser::new("med(5,2,8,9)", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 6);
    }
}
