# String Calculator

> A small package containing eval methods to compute formulas inside strings.

## How to use

Simply import the eval you need and use it.

```rs
use string_templater::eval_f64;

fn main() {
  println!("{}", eval_f64("(2+3) / 2".to_string(), 0.0)); // 2.5
}
```

## Operators

Since there's a lot of things that could be simplified visually, here's the full list of operators implemented.

1. Add (x+y)
  This operator handle the addition between two number.
  Example:
  `1 + 2`
  `= 3`
1. Subtract (x-y)
  This operator handle the subtraction between two number.
  Example:
  `2 - 1`
  `= 1`
1. Multiply (x*y)
  This operator handle the multiplication between two number.
  Example:
  `3 * 2`
  `= 6`
1. Divide (x/y)
  This operator handle the division between two number.
  Example:
  `4 / 2`
  `= 2`
1. Modulo (x%y)
  This operator handle the rest of the euclidian division between two number.
  Example:
  `4 % 2`
  `= 0`
1. Left Shift (x<<y) (only in `eval_i64`)
  This operator shift the bits of `x` to `y` positions to the left (it's the same as multiplying by `2^y`).
  Example:
  `1 << 2`
  `= 4`
1. Right Shift (x>>y) (only in `eval_i64`)
  This operator shift the bits of `x` to `y` positions to the right (it's the same as dividing by `2^y`).
  Example:
  `4 >> 2`
  `= 1`
1. PowerOf (x^y) (only in `eval_f64` and `eval_i64`)
  This operator handle the power of `x` by `y`, `x` and `y` being both numbers.
  Example:
  `3^3`
  `= 27`
1. Subscript support (x²) (only in `eval_f64` and `eval_i64`)
  This operator handle the power of `x` by using the superscript notation (such as `²`) for integers, `x` being a number.
  Example:
  `5²`
  `= 25`
1. Factorial (x!) (only in `eval_f64` and `eval_i64`)
  This operator handle the factorial of a real `x`.
  Example:
  `5!`
  `= 120`
1. DegToRad (x°) (only in `eval_f64`)
  This operator handle the conversion from degree to radian. You should note that it's priority is the same as multiplication.
  Example:
  `3°`
  `= 0.05235987755982989`
1. RadToDeg (x rad) (only in `eval_f64`)
  This operator handle the conversion from radian to degree. You should note that it's priority is the same as multiplication.
  Example:
  `3 rad`
  `= 171.8873385393`

## Function notation

Some function can be written purely using their original mathematical notation if wanted.

1. Floor (⌊x⌋) (only in `eval_f64` and `eval_decimal`)
  This function gives the greatest integer less than or equal to `x`.
  Example:
  `⌊2.4⌋`
  `= 2`
1. Ceiling (⌈x⌉) (only in `eval_f64` and `eval_decimal`)
  This function gives the smallest integer greater or equal to `x`.
  Example:
  `⌈2.4⌉`
  `= 3`

## Functions

1. Absolute value (abs(x))
1. Signum (sgn(x), sign(x), signum(x))
1. Power (pow(x,y)) (only in `eval_f64` and `eval_i64`)
1. Square root (sqrt(x)) (only in `eval_f64` and `eval_i64`)
1. Root (root(x)) (only in `eval_f64` and `eval_i64`)
1. Modulo (mod(x,y))
1. Exponential (exp(x), exp2(x)) (only in `eval_f64` and `eval_i64`)
1. Logarithm (ln(x), log(x, b)) (only in `eval_f64` and `eval_i64`)
1. Extremum (min(...X), max(...X))
1. Avg (avg(...X))
1. Median (median(...X), med(...X))
1. Truncate (trunc(x), truncate(x)) (only in `eval_f64` and `eval_decimal`)
1. Floor (floor(x)) (only in `eval_f64` and `eval_decimal`)
1. Ceil (ceil(x)) (only in `eval_f64` and `eval_decimal`)
1. Round (round(x)) (only in `eval_f64` and `eval_decimal`)
1. Sin (sin(θ)) (only in `eval_f64`)
1. Asin (asin(x)) (only in `eval_f64`)
1. cos (cos(θ)) (only in `eval_f64`)
1. Acos (acos(x)) (only in `eval_f64`)
1. Tan (tan(θ)) (only in `eval_f64`)
1. Atan (atan(x)) (only in `eval_f64`)
1. Sinh (sinh(θ)) (only in `eval_f64`)
1. Asinh (asinh(x), arsinh(x)) (only in `eval_f64`)
1. Cosh (cosh(θ)) (only in `eval_f64`)
1. Acosh (acosh(x), arcosh(x)) (only in `eval_f64`)
1. Tanh (tanh(θ)) (only in `eval_f64`)
1. Atanh (atanh(x), artanh(x)) (only in `eval_f64`)
1. Atan 2 (atan2(y, x)) (only in `eval_f64`)


## Placeholder Getter

In the case you're writting a calculator, it might be useful to use your previous answer for example.
The `@` symbol is used here as a placeholder for the value you want to put into the `eval_f64`, `eval_i64` or `eval_decimal`.
