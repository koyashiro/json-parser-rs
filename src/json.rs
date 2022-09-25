use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Number(f64),
    String(String),
}
