#[derive(Debug)]
pub enum Value {
    Integer(i32),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub fn add(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(*a + *b),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a + *b),
            _ => todo!(),
        }
    }

    pub fn uadd(&mut self) -> Self {
        match self {
            Value::Integer(a) => Value::Integer(*a),
            Value::Float(a) => Value::Float(*a),
            _ => todo!(),
        }
    }

    pub fn sub(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(*a - *b),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a - *b),
            _ => todo!(),
        }
    }

    pub fn usub(&mut self) -> Self {
        match self {
            Value::Integer(a) => Value::Integer(-*a),
            Value::Float(a) => Value::Float(-*a),
            _ => todo!(),
        }
    }

    pub fn mul(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(*a * *b),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a * *b),
            _ => todo!(),
        }
    }

    pub fn div(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => todo!(),
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(*a / *b),
            (Value::Float(_), Value::Float(b)) if b == &0.0 => todo!(),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a / *b),
            _ => todo!(),
        }
    }

    pub fn modulo(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(*a % *b),
            (Value::Float(a), Value::Float(b)) => Value::Float(*a % *b),
            _ => todo!(),
        }
    }

    pub fn pow(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) if *b >= 0 => {
                Value::Integer((*a as f64).powi(*b as i32) as i32)
            }
            (Value::Integer(_), Value::Integer(_)) => todo!(),
            (Value::Integer(a), Value::Float(b)) => Value::Float((*a as f64).powf(*b)),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a.powf(*b as f64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(*b)),
            _ => todo!(),
        }
    }

    pub fn not(&mut self) -> Self {
        match self {
            Value::Boolean(a) => Value::Boolean(!*a),
            _ => todo!(),
        }
    }

    pub fn and(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a && *b),
            _ => todo!(),
        }
    }

    pub fn or(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a || *b),
            _ => todo!(),
        }
    }

    pub fn equals(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a == *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a == *b),
            //       True  False
            // True  True  False
            // False False True
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(!(*a ^ *b)),
            _ => todo!(),
        }
    }

    pub fn not_equals(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a != *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a != *b),
            //       True  False
            // True  True  False
            // False False True
            (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(*a ^ *b),
            _ => todo!(),
        }
    }

    pub fn greater(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a > *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a > *b),
            _ => todo!(),
        }
    }

    pub fn greater_eq(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a >= *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a >= *b),
            _ => todo!(),
        }
    }

    pub fn less_than(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a < *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a < *b),
            _ => todo!(),
        }
    }

    pub fn less_than_eq(&mut self, other: &Self) -> Self {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Value::Boolean(*a <= *b),
            (Value::Float(a), Value::Float(b)) => Value::Boolean(*a <= *b),
            _ => todo!(),
        }
    }
}
