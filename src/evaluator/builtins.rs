use crate::Value;
use std::process;

pub fn builtin(name: &str, arg_names: Vec<&str>, func: fn(Vec<Value>) -> Value) -> (String, Value) {
    let arguments: Vec<String> = arg_names.into_iter().map(ToString::to_string).collect();
    (
        name.to_string(),
        Value::BuiltInFunction {
            name: name.to_string(),
            arguments: arguments,
            function: func,
        },
    )
}

pub fn print(values: Vec<Value>) -> Value {
    print!("{}\n", values[0]);
    Value::None
}

pub fn exit(values: Vec<Value>) -> Value {
    if values.len() != 1 {
        todo!()
    } else if let Value::Integer(int) = values[0] {
        if int >= 0 && int <= 255 {
            process::exit(int);
        } else {
            todo!()
        }
    }
    Value::None
}
