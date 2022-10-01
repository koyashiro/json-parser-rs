use std::collections::HashMap;

use crate::{error::Error, json::Value, tokenize::Token};

pub fn parse(tokens: &[Token]) -> Result<Value, Error> {
    let mut p = tokens;
    let value = parse_value(&mut p)?;
    if !p.is_empty() {
        return Err(Error::UnexpectedNonWhitespace);
    }
    Ok(value)
}

fn parse_value(tokens: &mut &[Token]) -> Result<Value, Error> {
    match tokens.first() {
        Some(Token::Null) => {
            *tokens = &tokens[1..];
            Ok(Value::Null)
        }
        Some(Token::False) => {
            *tokens = &tokens[1..];
            Ok(Value::Bool(false))
        }
        Some(Token::True) => {
            *tokens = &tokens[1..];
            Ok(Value::Bool(true))
        }
        Some(Token::Number(n)) => {
            *tokens = &tokens[1..];
            Ok(Value::Number(*n))
        }
        Some(Token::String(s)) => {
            *tokens = &tokens[1..];
            Ok(Value::String(s.to_string()))
        }
        Some(Token::BeginObject) => parse_object(tokens),
        Some(Token::BeginArray) => parse_array(tokens),
        Some(t) => Err(Error::UnexpectedToken(t.to_string())),
        None => Err(Error::UnexpectedEnd),
    }
}

fn parse_object(tokens: &mut &[Token]) -> Result<Value, Error> {
    match tokens.first() {
        Some(Token::BeginObject) => *tokens = &tokens[1..],
        Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
        None => return Err(Error::UnexpectedEnd),
    }

    let mut o = HashMap::new();

    loop {
        if let Some(Token::EndObject) = tokens.first() {
            *tokens = &tokens[1..];
            return Ok(Value::Object(o));
        }

        if !o.is_empty() {
            match tokens.first() {
                Some(Token::ValueSeparator) => *tokens = &tokens[1..],
                Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
                None => return Err(Error::UnexpectedEnd),
            }
        }

        let k = match tokens.first() {
            Some(Token::String(k)) => {
                *tokens = &tokens[1..];
                k.to_string()
            }
            Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
            None => return Err(Error::UnexpectedEnd),
        };

        match tokens.first() {
            Some(Token::NameSeparator) => *tokens = &tokens[1..],
            Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
            None => return Err(Error::UnexpectedEnd),
        }

        let v = parse_value(tokens)?;

        o.insert(k, v);
    }
}

fn parse_array(tokens: &mut &[Token]) -> Result<Value, Error> {
    match tokens.first() {
        Some(Token::BeginArray) => *tokens = &tokens[1..],
        Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
        None => return Err(Error::UnexpectedEnd),
    }

    let mut a = Vec::new();
    loop {
        if let Some(Token::EndArray) = tokens.first() {
            *tokens = &tokens[1..];
            return Ok(Value::Array(a));
        }

        if !a.is_empty() {
            match tokens.first() {
                Some(Token::ValueSeparator) => *tokens = &tokens[1..],
                Some(t) => return Err(Error::UnexpectedToken(t.to_string())),
                None => return Err(Error::UnexpectedEnd),
            }
        }

        let v = parse_value(tokens)?;
        a.push(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value_test() {
        assert_eq!(
            parse_value(&mut vec![Token::Null].as_slice()),
            Ok(Value::Null)
        );
        assert_eq!(
            parse_value(&mut vec![Token::False].as_slice()),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            parse_value(&mut vec![Token::True].as_slice()),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            parse_value(&mut vec![Token::Number(123.45)].as_slice()),
            Ok(Value::Number(123.45))
        );
        assert_eq!(
            parse_value(&mut vec![Token::String("value")].as_slice()),
            Ok(Value::String("value".to_string()))
        );
        assert_eq!(
            parse_value(
                &mut vec![
                    Token::BeginObject,
                    Token::String("keyA"),
                    Token::NameSeparator,
                    Token::String("valueA"),
                    Token::EndObject,
                ]
                .as_slice()
            ),
            Ok(Value::Object(
                [("keyA".to_string(), Value::String("valueA".to_string()))].into()
            ))
        );
        assert_eq!(
            parse_value(
                &mut vec![
                    Token::BeginArray,
                    Token::String("value1"),
                    Token::ValueSeparator,
                    Token::String("value2"),
                    Token::ValueSeparator,
                    Token::String("value3"),
                    Token::EndArray,
                ]
                .as_slice()
            ),
            Ok(Value::Array(vec![
                Value::String("value1".to_string()),
                Value::String("value2".to_string()),
                Value::String("value3".to_string()),
            ]))
        );
    }
}
