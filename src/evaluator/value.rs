use crate::models::error::{Error, Runtime};
use crate::tokenizer::token::Token;
use crate::tokenizer::token::TokenKind;

/*
The `invalid_operation` macro takes an operator, right and
left tokens, and creates and invalid operation error wrapped
in an Err out of the three.
*/
macro_rules! invalid_operation {
    ($operator:expr, $right:expr, $left:expr) => {
        Err(Error::runtime(Runtime::InvalidOperation {
            operator: $operator,
            right: $right,
            left: $left,
        }))
    };
}

/*
--- Value (enum) ---

The Value enum is an enum containing all the possible values
that can be evaluated in the Evaluator, this enum makes it
possible to return a single object that contains a result
for an expression.
*/
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f32),
    Boolean(bool),
}

// Operation is the return type of all operations functions.
type Operation = Result<Value, Error>;

impl Value {
    /*
    The from function takes a token and creates a correct Value
    type based on the token kind.
    */
    pub fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Integer(value) => Self::Integer(value),
            TokenKind::Float(value) => Self::Float(value),
            TokenKind::Boolean(value) => Self::Boolean(value),
            _ => unreachable!(),
        }
    }

    /*
    The binary_operation method takes an other Value type and an
    operator token, then it matches the given operator kind to
    preforms the correct function on the Value the method was
    called on.
    */
    pub fn binary_operation(self, other: Self, operator: Token) -> Operation {
        match operator.kind {
            TokenKind::Add => self.add(other, operator),
            TokenKind::Sub => self.sub(other, operator),
            TokenKind::Mul => self.mul(other, operator),
            TokenKind::Div => self.div(other, operator),
            TokenKind::Mod => self.modulo(other, operator),
            TokenKind::Pow => self.pow(other, operator),
            TokenKind::And => self.and(other, operator),
            TokenKind::Or => self.or(other, operator),
            TokenKind::Equals => self.equals(other, operator),
            TokenKind::NotEquals => self.not_equals(other, operator),
            TokenKind::Greater => self.greater(other, operator),
            TokenKind::GreaterEq => self.greater_eq(other, operator),
            TokenKind::LessThan => self.less_than(other, operator),
            TokenKind::LessThanEq => self.less_than_eq(other, operator),
            _ => unreachable!(),
        }
    }

    /*
    The unary_operation method takes an operator token, and
    based on his kind preforms the correct function on the Value
    the method was called on.
    */
    pub fn unary_operation(self, operator: Token) -> Operation {
        match operator.kind {
            TokenKind::Add => self.uadd(operator),
            TokenKind::Sub => self.usub(operator),
            TokenKind::Not => self.not(operator),
            _ => unreachable!(),
        }
    }

    /*
    The add method takes an other Value and the specific 'Add'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn add(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The uadd (unary add) method takes the specific 'Add' token,
    then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn uadd(self, operator: Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(a)),
            Value::Float(a) => Ok(Value::Float(a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    /*
    The sub method takes an other Value and the specific 'Sub'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn sub(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The usub (unary sub) method takes the specific 'Sub' token,
    then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn usub(self, operator: Token) -> Operation {
        match self {
            Value::Integer(a) => Ok(Value::Integer(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    /*
    The mul method takes an other Value and the specific 'Mul'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn mul(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The div method takes an other Value and the specific 'Div'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error. Furthermore, in a case where
    the numerator is an Integer or a Float but the numerator
    is an Integer(0) or Float(0) the method returns a division
    by zero error.
    */
    fn div(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(_), Value::Integer(0)) => {
                Err(Error::runtime(Runtime::DivisionByZero {
                    division: operator,
                }))
            }
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a / b)),
            (Value::Float(_), Value::Float(b)) if b == 0.0 => {
                Err(Error::runtime(Runtime::DivisionByZero {
                    division: operator,
                }))
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The modulo method takes an other Value and the specific
    'Mod' token, then using a match statement figures out the
    correct operation to preform. If no block was matched, it
    returns an invalid operation error.
    */
    fn modulo(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The pow method takes an other Value and the specific 'Pow'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn pow(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) if b >= 0 => {
                Ok(Value::Integer((a as f64).powi(b as i32) as i32))
            }
            (Value::Integer(_), Value::Integer(_)) => todo!(),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((a as f32).powf(b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(b as f32))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(b))),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The not method takes the specific 'Not' token, then using a
    match statement figures out the correct operation to
    preform. If no block was matched, it returns an invalid
    operation error.
    */
    fn not(self, operator: Token) -> Operation {
        match self {
            Value::Boolean(a) => Ok(Value::Boolean(!a)),
            left => invalid_operation!(operator, None, left),
        }
    }

    /*
    The and method takes an other Value and the specific 'And'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn and(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a && b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The or method takes an other Value and the specific 'Or'
    token, then using a match statement figures out the correct
    operation to preform. If no block was matched, it returns
    an invalid operation error.
    */
    fn or(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a || b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The equals method takes an other Value and the specific
    'Equals' token, then using a match statement figures out the
    correct operation to preform. If no block was matched, it
    returns an invalid operation error.
    */
    fn equals(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a == b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The not_equals method takes an other Value and the specific
    'NotEquals' token, then using a match statement figures out
    the correct operation to preform. If no block was matched,
    it returns an invalid operation error.
    */
    fn not_equals(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a != b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(Value::Boolean(a ^ b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The greater method takes an other Value and the specific
    'Greater' token, then using a match statement figures out
    the correct operation to preform. If no block was matched,
    it returns an invalid operation error.
    */
    fn greater(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a > b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The greater_eq method takes an other Value and the specific
    'GreaterEq' token, then using a match statement figures out
    the correct operation to preform. If no block was matched,
    it returns an invalid operation error.
    */
    fn greater_eq(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a >= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The less_than method takes an other Value and the specific
    'LessThan' token, then using a match statement figures out
    the correct operation to preform. If no block was matched,
    it returns an invalid operation error.
    */
    fn less_than(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a < b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }

    /*
    The less_than_eq method takes an other Value and the
    specific 'LessThanEq' token, then using a match statement
    figures out the correct operation to preform. If no block
    was matched, it returns an invalid operation error.
    */
    fn less_than_eq(self, other: Self, operator: Token) -> Operation {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Boolean(a <= b)),
            (right, left) => invalid_operation!(operator, Some(right), left),
        }
    }
}
