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
    Ln(Box<Node>),
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Log(Box<Node>, Box<Node>),
    Number(Decimal),
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
        Exp(sub_expr) => Ok(eval(*sub_expr)?.exp()),
        Exp2(sub_expr) => Ok(Decimal::new(2, 0).powd(eval(*sub_expr)?)),
        Pow(expr1, expr2) => Ok(eval(*expr1)?.powd(eval(*expr2)?)),
        Log(expr1, expr2) => Ok(eval(*expr1)?.ln() / eval(*expr2)?.ln()),
        Sqrt(sub_expr) => {
            match eval(*sub_expr)?.sqrt() {
                Some(result) => Ok(result),
                None => Err("Unable to compute the square root of negative number".into()),
            }
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
