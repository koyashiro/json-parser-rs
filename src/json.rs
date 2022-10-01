use std::{collections::HashMap, str::FromStr};

use crate::{error::Error, parse::parse, tokenize::tokenize};

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Number(f64),
    String(String),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = tokenize(s)?;
        let value = parse(&tokens)?;
        Ok(value)
    }
}
