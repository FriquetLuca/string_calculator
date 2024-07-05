#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Float(f64),
    Integer(i64),
    Complex(Box<Number>, Box<Number>),
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
      Number::Integer(value)
    }
}

impl From<(i64, i64)> for Number {
    fn from(value: (i64, i64)) -> Self {
      Number::Complex(Box::new(Number::Integer(value.0)), Box::new(Number::Integer(value.1)))
    }
}

impl From<f64> for Number {
  fn from(value: f64) -> Self {
    let floored_value = value.floor();
    if (value - floored_value) == 0.0 {
      if floored_value >= (i64::MIN as f64) && floored_value <= (i64::MAX as f64) {
        Number::Integer(floored_value as i64)
      } else {
        Number::Float(value)
      }
    } else {
      Number::Float(value)
    }
  }
}

impl From<(f64, f64)> for Number {
    fn from(value: (f64, f64)) -> Self {
      Number::Complex(Box::new(Number::Float(value.0)), Box::new(Number::Float(value.1)))
    }
}

impl From<(i64, f64)> for Number {
    fn from(value: (i64, f64)) -> Self {
      Number::Complex(Box::new(Number::Integer(value.0)), Box::new(Number::Float(value.1)))
    }
}

impl From<(f64, i64)> for Number {
    fn from(value: (f64, i64)) -> Self {
      Number::Complex(Box::new(Number::Float(value.0)), Box::new(Number::Integer(value.1)))
    }
}
