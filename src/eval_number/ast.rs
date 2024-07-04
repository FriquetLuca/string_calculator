use std::{error, sync::Arc};

use super::token::Number;

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
    ILog(Box<Node>, Box<Node>),
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
    Lb(Box<Node>),
    Exp(Box<Node>),
    Exp2(Box<Node>),
    Truncate(Box<Node>),
    Sign(Box<Node>),
    LambertW(Box<Node>),
    Min(Arc<Vec<Node>>),
    Max(Arc<Vec<Node>>),
    Avg(Arc<Vec<Node>>),
    Med(Arc<Vec<Node>>),
    Num(Number),
}

fn gamma(a: f64) -> f64 {
    let mut s = 2.4857408913875355e-5;
    if a < 0.5 {
        s += 1.0514237858172197 / (1.0 - a);
        s += -3.4568709722201625 / (2.0 - a);
        s += 4.512277094668948 / (3.0 - a);
        s += -2.982852253_2357664 / (4.0 - a);
        s += 1.056397115771267 / (5.0 - a);
        s += -1.9542877319164587e-1 / (6.0 - a);
        s += 1.709705434044412e-2 / (7.0 - a);
        s += -5.719261174043057e-4 / (8.0 - a);
        s += 4.633994733599057e-6 / (9.0 - a);
        s += -2.7199490848860772e-9 / (10.0 - a);
        std::f64::consts::PI
            / ((std::f64::consts::PI * a).sin()
                * s
                * 1.860_382_734_205_265_7
                * ((a - 10.400511) / std::f64::consts::E).powf(0.5 - a))
    } else {
        s += 1.0514237858172197 / a;
        s += -3.456870972220_1625 / (a + 1.0);
        s += 4.512277094668948 / (a + 2.0);
        s += -2.9828522532357664 / (a + 3.0);
        s += 1.056397115771267 / (a + 4.0);
        s += -1.9542877319164587e-1 / (a + 5.0);
        s += 1.709705434044412e-2 / (a + 6.0);
        s += -5.719261174043057e-4 / (a + 7.0);
        s += 4.633994733599057e-6 / (a + 8.0);
        s += -2.7199490848860772e-9 / (a + 9.0);
        s * 1.8603827342052657 * ((a + 10.400511) / std::f64::consts::E).powf(a - 0.5)
    }
}

pub fn eval(expr: Node) -> Result<Number, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Num(i) => Ok(i),
        Add(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => {
                        match value_a.checked_add(value_b) {
                            Some(sum) => Ok(Number::Integer(sum)),
                            None => Ok(Number::Float((value_a as f64) + (value_b as f64))),
                        }
                    },
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) + value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a + (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a + value_b)),
                }
            }
        },
        Subtract(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => {
                        match value_a.checked_sub(value_b) {
                            Some(sub) => Ok(Number::Integer(sub)),
                            None => Ok(Number::Float((value_a as f64) - (value_b as f64))),
                        }
                    },
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) - value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a - (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a - value_b)),
                }
            }
        },
        Multiply(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => {
                        match value_a.checked_mul(value_b) {
                            Some(sub) => Ok(Number::Integer(sub)),
                            None => Ok(Number::Float((value_a as f64) * (value_b as f64))),
                        }
                    },
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) * value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a * (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a * value_b)),
                }
            }
        },
        Divide(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => {
                        match value_a.checked_rem_euclid(value_b) {
                            Some(r) => if r == 0 {
                                Ok(Number::Integer(value_a / value_b))
                            } else {
                                Ok(Number::Float((value_a as f64) / (value_b as f64)))
                            }
                            None => Ok(Number::Float((value_a as f64) / (value_b as f64)))
                        }
                    },
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) / value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a / (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a / value_b)),
                }
            }
        },
        Modulo(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Integer(value_a % value_b)),
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) % value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a % (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a % value_b)),
                }
            }
        },
        Negative(expr1) => {
            let x = eval(*expr1)?;
            match x {
                Number::Integer(v) => match (0 as i64).checked_sub(v) {
                    Some(neg) => Ok(Number::Integer(neg)),
                    None => Ok(Number::Float(-(v as f64))),
                }
                Number::Float(v) => Ok(Number::Float(-v)),
            }
        },
        Pow(expr1, expr2) => {
            let a = eval(*expr1)?;
            let b = eval(*expr2)?;
            match a {
                Number::Integer(value_a) => match b {
                    Number::Integer(value_b) => {
                        if value_b >= 0 {
                            if value_b <= (u32::MAX as i64) {
                                match value_a.checked_pow(value_b as u32) {
                                    Some(p) => Ok(Number::Integer(p)),
                                    None => Ok(Number::Float((value_a as f64).powf(value_b as f64))),
                                }
                            } else {
                                Ok(Number::Float((value_a as f64).powf(value_b as f64)))
                            }
                        } else {
                            let value_a = value_a as f64;
                            if value_b < (i32::MIN as i64) {
                                Ok(Number::Float(value_a.powf(value_b as f64)))
                            } else {
                                Ok(Number::Float(value_a.powi(value_b as i32)))
                            }
                        }
                    },
                    Number::Float(value_b) => Ok(Number::Float((value_a as f64) / value_b)),
                }
                Number::Float(value_a) => match b {
                    Number::Integer(value_b) => Ok(Number::Float(value_a / (value_b as f64))),
                    Number::Float(value_b) => Ok(Number::Float(value_a / value_b)),
                }
            }
        },
        Root(n_th_expr, x_expr) => {
            let x = eval(*x_expr)?;
            let root = eval(*n_th_expr)?;
            match root {
                Number::Integer(n) => match x {
                    Number::Integer(x) => Ok(Number::Float((x as f64).powf(1.0 / (n as f64)))),
                    Number::Float(x) => Ok(Number::Float(x.powf(1.0 / (n as f64)))),
                }
                Number::Float(n) => match x {
                    Number::Integer(x) => Ok(Number::Float((x as f64).powf(1.0 / n))),
                    Number::Float(x) => Ok(Number::Float(x.powf(1.0 / n))),
                }
            }
        },
        Factorial(sub_expr) => {
            let sub_result = eval(*sub_expr)?;
            match sub_result {
                Number::Integer(n) => {
                    if n <= 20 && n >= 0 {
                        let mut factorial_result = 1;
                        for i in 2..=(n as usize) {
                            factorial_result *= i as i64;
                        }
                        Ok(Number::Integer(factorial_result))
                    } else {
                        Ok(Number::Float(gamma((n as f64) + 1.0)))
                    }
                }
                Number::Float(n) => {
                    Ok(Number::Float(gamma(n + 1.0)))
                }
            }
        }
        LambertW(expr) => {
            let sub_expr = eval(*expr)?;
            let sub_expr = match sub_expr {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            let min_one: f64 = -1.0;
            if sub_expr < -min_one.exp() {
                return Err("The Lambert W function is not defined for {}.".into());
            }
            let iterations = (4).max((sub_expr.log10() / 3.0).ceil() as i32);
            let mut w: f64 = 0.0;
            for _ in 0..iterations {
                let exp_w = w.exp();
                w -= (w * exp_w - sub_expr)
                    / (exp_w * (w + 1.0) - (w + 2.0) * (w * exp_w - sub_expr) / (2.0 * w + 2.0));
            }
            Ok(Number::Float(w))
        }
        ILog(expr1, expr2) => {
            let n = eval(*expr1)?;
            let mut n = match n {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            let b = eval(*expr2)?;
            let b = match b {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            let mut x: i64 = 0;
            while n > 1.0 {
                x += 1;
                n = (n.log10() / b.log10()).floor();
            }
            Ok(Number::Integer(x))
        }
        Abs(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(x) => match x.checked_abs() {
                    Some(x) => Ok(Number::Integer(x)),
                    None => Ok(Number::Float((x as f64).abs()))
                }
                Number::Float(x) => Ok(Number::Float(x.abs())),
            }
        },
        Floor(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(n) => Ok(Number::Integer(n)),
                Number::Float(n) => {
                    let f = n.floor();
                    if (f <= (i64::MAX as f64)) && (f >= (i64::MIN as f64)) {
                        Ok(Number::Integer(n as i64))
                    } else {
                        Ok(Number::Float(f))
                    }
                }
            }
        },
        Ceil(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(n) => Ok(Number::Integer(n)),
                Number::Float(n) => {
                    let f = n.ceil();
                    if (f <= (i64::MAX as f64)) && (f >= (i64::MIN as f64)) {
                        Ok(Number::Integer(n as i64))
                    } else {
                        Ok(Number::Float(f))
                    }
                }
            }
        },
        Round(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(n) => Ok(Number::Integer(n)),
                Number::Float(n) => {
                    let f = n.round();
                    if (f <= (i64::MAX as f64)) && (f >= (i64::MIN as f64)) {
                        Ok(Number::Integer(n as i64))
                    } else {
                        Ok(Number::Float(f))
                    }
                }
            }
        },
        Sin(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.sin())),
                Number::Integer(i) => Ok(Number::Float((i as f64).sin()))
            }
        },
        Cos(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.cos())),
                Number::Integer(i) => Ok(Number::Float((i as f64).cos()))
            }
        },
        Tan(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.tan())),
                Number::Integer(i) => Ok(Number::Float((i as f64).tan()))
            }
        },
        Sinh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.sinh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).sinh()))
            }
        },
        Cosh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.cosh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).cosh()))
            }
        },
        Tanh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.tanh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).tanh()))
            }
        },
        Asin(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.asin())),
                Number::Integer(i) => Ok(Number::Float((i as f64).asin()))
            }
        },
        Acos(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.acos())),
                Number::Integer(i) => Ok(Number::Float((i as f64).acos()))
            }
        },
        Atan(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.atan())),
                Number::Integer(i) => Ok(Number::Float((i as f64).atan()))
            }
        },
        Arsinh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.asinh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).asinh()))
            }
        },
        Arcosh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.acosh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).acosh()))
            }
        },
        Artanh(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            match sub_expr {
                Number::Float(f) => Ok(Number::Float(f.atanh())),
                Number::Integer(i) => Ok(Number::Float((i as f64).atanh()))
            }
        },
        Sqrt(sub_expr) => { // @todo: check if it's a perfect square, if so and a i64, then cast to integer
            let sqr = eval(*sub_expr)?;
            match sqr {
                Number::Integer(i) => Ok(Number::Float((i as f64).sqrt())),
                Number::Float(f) => Ok(Number::Float(f.sqrt())),
            }
        },
        Ln(sub_expr) => {
            let sqr = eval(*sub_expr)?;
            match sqr {
                Number::Integer(i) => Ok(Number::Float((i as f64).ln())),
                Number::Float(f) => Ok(Number::Float(f.ln())),
            }
        },
        Lb(sub_expr) => {
            let sqr = eval(*sub_expr)?;
            match sqr {
                Number::Integer(i) => Ok(Number::Float((i as f64).log(2.0))),
                Number::Float(f) => Ok(Number::Float(f.log(2.0))),
            }
        },
        Truncate(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(n) => Ok(Number::Integer(n)),
                Number::Float(n) => {
                    let f = n.trunc();
                    if (f <= (i64::MAX as f64)) && (f >= (i64::MIN as f64)) {
                        Ok(Number::Integer(n as i64))
                    } else {
                        Ok(Number::Float(f))
                    }
                }
            }
        },
        Sign(sub_expr) => {
            let x = eval(*sub_expr)?;
            match x {
                Number::Integer(n) => Ok(Number::Integer(n.signum())),
                Number::Float(n) => {
                    if n > 0.0 {
                        Ok(Number::Integer(1))
                    } else if n == 0.0 {
                        Ok(Number::Integer(0))
                    } else {
                        Ok(Number::Integer(-1))
                    }
                }
            }
        },
        Exp(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            let sub_expr = match sub_expr {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            Ok(Number::Float(sub_expr.exp()))
        },
        Exp2(sub_expr) => {
            let sub_expr = eval(*sub_expr)?;
            let sub_expr = match sub_expr {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            Ok(Number::Float(sub_expr.exp2()))
        },
        Log(expr1, expr2) => {
            let expr1 = eval(*expr1)?;
            let expr1 = match expr1 {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            let expr2 = eval(*expr2)?;
            let expr2 = match expr2 {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            Ok(Number::Float(expr1.log(expr2)))
        },
        Min(args) => {
            if args.len() > 1 {
                let mut result: Option<Number> = None;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    let r = eval(arg)?;
                    match result {
                        Some(l) => {
                            let lf64 = match l.clone() {
                                Number::Float(f) => f,
                                Number::Integer(i) => i as f64,
                            };
                            let rf64 = match r.clone() {
                                Number::Float(f) => f,
                                Number::Integer(i) => i as f64,
                            };
                            if lf64 < rf64 {
                                result = Some(l);
                            } else {
                                result = Some(r);
                            }
                        }
                        None => {
                            result = Some(r);
                        }
                    }
                }
                Ok(result.unwrap())
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(Number::Integer(0)),
                }
            }
        }
        Max(args) => {
            if args.len() > 1 {
                let mut result: Option<Number> = None;
                for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                    let r = eval(arg)?;
                    match result {
                        Some(l) => {
                            let lf64 = match l.clone() {
                                Number::Float(f) => f,
                                Number::Integer(i) => i as f64,
                            };
                            let rf64 = match r.clone() {
                                Number::Float(f) => f,
                                Number::Integer(i) => i as f64,
                            };
                            if lf64 > rf64 {
                                result = Some(l);
                            } else {
                                result = Some(r);
                            }
                        }
                        None => {
                            result = Some(r);
                        }
                    }
                }
                Ok(result.unwrap())
            } else {
                match args.first() {
                    Some(arg) => Ok(eval((*arg).clone())?),
                    None => Ok(Number::Integer(0)),
                }
            }
        }
        Avg(args) => {
            let mut result = 0.0;
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                let sub_expr = eval(arg)?;
                let sub_expr = match sub_expr {
                    Number::Integer(x) => x as f64,
                    Number::Float(x) => x,
                };
                result += sub_expr;
            }
            let len = args.len() as f64;
            Ok(Number::Float(result / len))
        }
        Med(args) => {
            let mut results = vec![];
            for arg in <Vec<Node> as Clone>::clone(&args).into_iter() {
                results.push(eval(arg).unwrap());
            }
            results.sort_by(|a, b| {
                let a = match a {
                    Number::Integer(x) => (*x) as f64,
                    Number::Float(x) => *x,
                };
                let b = match b {
                    Number::Integer(x) => (*x) as f64,
                    Number::Float(x) => *x,
                };
                a.partial_cmp(&b).unwrap()
            });
            let len = results.len();
            if len % 2 == 0 {
                let a = results[len >> 1].clone();
                let a = match a {
                    Number::Integer(x) => x as f64,
                    Number::Float(x) => x,
                };
                let b = results[(len >> 1) - 1].clone();
                let b = match b {
                    Number::Integer(x) => x as f64,
                    Number::Float(x) => x,
                };
                Ok(Number::Float((a + b) / 2.0))
            } else {
                Ok(results[len >> 1].clone())
            }
        }
        Atan2(expr1, expr2) => {
            let expr1 = eval(*expr1)?;
            let expr1 = match expr1 {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            let expr2 = eval(*expr2)?;
            let expr2 = match expr2 {
                Number::Integer(x) => x as f64,
                Number::Float(x) => x,
            };
            Ok(Number::Float(expr1.atan2(expr2)))
        },
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use crate::eval_number::{ast::eval, parser::Parser, token::Number};

    #[test]
    fn test_div() {
        let ast = Parser::new("1/2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Float(0.5));
    }
    #[test]
    fn test_modulo() {
        let ast = Parser::new("1%2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(1));
    }
    #[test]
    fn test_modulo2() {
        let ast = Parser::new("2%2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(0));
    }
    #[test]
    fn test_modulo3() {
        let ast = Parser::new("3%2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(1));
    }
    #[test]
    fn test_expr1() {
        let ast = Parser::new("1+2-3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(0));
    }
    #[test]
    fn test_expr2() {
        let ast = Parser::new("3+2-1*5/4", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Float(3.75));
    }
    #[test]
    fn test_expr3() {
        let ast = Parser::new("5+(2*7-3!)*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(29));
    }
    #[test]
    fn test_expr4() {
        let ast = Parser::new("2*4%3/2", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(1));
    }
    #[test]
    fn test_expr5() {
        let ast = Parser::new("3*2^3*3", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(72));
    }
    #[test]
    fn test_expr6() {
        let ast = Parser::new("2+3*atan2(3,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Float(2.0 + 3.0 * (3.0 as f64).atan2(7.0)));
    }
    #[test]
    fn test_expr7() {
        let ast = Parser::new("med(5,2,8,9,7)", None)
            .unwrap()
            .parse()
            .unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Integer(7));
    }
    #[test]
    fn test_expr8() {
        let ast = Parser::new("med(5,2,8,9)", None).unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, Number::Float(6.5));
    }
}
