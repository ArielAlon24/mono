use crate::Value;

pub fn print(values: Vec<Value>) -> Value {
    print!("{}\n", values[0]);
    Value::None
}
