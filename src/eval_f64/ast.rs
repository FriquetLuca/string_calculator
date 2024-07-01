use conv::ValueFrom;
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
    Number(f64),
}

fn gamma(a: f64) -> f64 {
    let mut s = 2.485_740_891_387_535_5e-5;
    if a < 0.5 {
        s += 1.051_423_785_817_219_7 / (1.0 - a);
        s += -3.456_870_972_220_162_5 / (2.0 - a);
        s += 4.512_277_094_668_948 / (3.0 - a);
        s += -2.982_852_253_235_766_4 / (4.0 - a);
        s += 1.056_397_115_771_267 / (5.0 - a);
        s += -1.954_287_731_916_458_7e-1 / (6.0 - a);
        s += 1.709_705_434_044_412e-2 / (7.0 - a);
        s += -5.719_261_174_043_057e-4 / (8.0 - a);
        s += 4.633_994_733_599_057e-6 / (9.0 - a);
        s += -2.719_949_084_886_077_2e-9 / (10.0 - a);
        std::f64::consts::PI
            / ((std::f64::consts::PI * a).sin()
                * s
                * 1.860_382_734_205_265_7
                * ((a - 10.400511) / std::f64::consts::E).powf(0.5 - a))
    } else {
        s += 1.051_423_785_817_219_7 / a;
        s += -3.456_870_972_220_162_5 / (a + 1.0);
        s += 4.512_277_094_668_948 / (a + 2.0);
        s += -2.982_852_253_235_766_4 / (a + 3.0);
        s += 1.056_397_115_771_267 / (a + 4.0);
        s += -1.954_287_731_916_458_7e-1 / (a + 5.0);
        s += 1.709_705_434_044_412e-2 / (a + 6.0);
        s += -5.719_261_174_043_057e-4 / (a + 7.0);
        s += 4.633_994_733_599_057e-6 / (a + 8.0);
        s += -2.719_949_084_886_077_2e-9 / (a + 9.0);
        s * 1.8603827342052657 * ((a + 10.400511) / std::f64::consts::E).powf(a - 0.5)
    }
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
        LambertW(expr) => {
            let sub_expr = eval(*expr)?;
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
            Ok(w)
        }
        ILog(expr1, expr2) => {
            let mut n = eval(*expr1)?;
            let b = eval(*expr2)?;
            let mut x: f64 = 0.0;
            while n > 1.0 {
                x += 1.0;
                n = (n.log10() / b.log10()).floor();
            }
            Ok(x)
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
        Lb(sub_expr) => Ok(eval(*sub_expr)?.log(2.0)),
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
