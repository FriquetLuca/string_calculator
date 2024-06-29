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
