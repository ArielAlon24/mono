use crate::Value;
use std::io;
use std::io::Write;
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

pub fn println(values: Vec<Value>) -> Value {
    if values.len() != 1 {
        todo!()
    }
    print!("{}\n", values[0]);
    Value::None
}

pub fn print(values: Vec<Value>) -> Value {
    if values.len() != 1 {
        todo!()
    }

    print!("{}", values[0]);
    io::stdout().flush().unwrap();
    Value::None
}

pub fn input(values: Vec<Value>) -> Value {
    if values.len() != 0 {
        todo!()
    }
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        return Value::String(input.trim_end().to_owned());
    }
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

pub fn integer(values: Vec<Value>) -> Value {
    if values.len() != 1 {
        todo!()
    }
    match &values[0] {
        Value::String(value) => match value.parse::<i32>() {
            Ok(integer) => Value::Integer(integer),
            Err(_) => Value::None,
        },
        _ => Value::None,
    }
}

pub fn string(values: Vec<Value>) -> Value {
    if values.len() != 1 {
        todo!()
    }
    Value::String(format!("{}", values[0]))
}
