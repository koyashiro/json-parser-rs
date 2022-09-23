use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    BeginArray,
    BeginObject,
    EndArray,
    EndObject,
    NameSeparator,
    ValueSeparator,
    False,
    Null,
    True,
    Number(f64),
    String(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    let chars = input.chars().collect::<Vec<char>>();
    let mut p = chars.as_slice();

    while let Some(c) = p.first() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                p = &p[1..];
            }
            '[' => {
                tokens.push(Token::BeginArray);
                p = &p[1..];
            }
            '{' => {
                tokens.push(Token::BeginObject);
                p = &p[1..];
            }
            ']' => {
                tokens.push(Token::EndArray);
                p = &p[1..];
            }
            '}' => {
                tokens.push(Token::EndObject);
                p = &p[1..];
            }
            ':' => {
                tokens.push(Token::NameSeparator);
                p = &p[1..];
            }
            ',' => {
                tokens.push(Token::ValueSeparator);
                p = &p[1..];
            }
            '"' => {
                p = &p[1..];

                let mut s = String::new();
                loop {
                    match p.first() {
                        Some('"') => {
                            tokens.push(Token::String(s));
                            p = &p[1..];
                            break;
                        }
                        Some(c) => {
                            s.push(*c);
                            p = &p[1..];
                        }
                        None => {
                            return Err(Error);
                        }
                    }
                }
            }
            '-' | '0'..='9' => {
                let mut s = String::new();

                if let Some(c @ '-') = p.first() {
                    s.push(*c);
                    p = &p[1..];
                }

                while let Some(c @ '0'..='9') = p.first() {
                    s.push(*c);
                    p = &p[1..];
                }

                if let Some(c @ '.') = p.first() {
                    s.push(*c);
                    p = &p[1..];

                    while let Some(c @ '0'..='9') = p.first() {
                        s.push(*c);
                        p = &p[1..];
                    }
                }

                if let Some(c @ 'e') = p.first() {
                    s.push(*c);
                    p = &p[1..];

                    if let Some(c @ '-') = p.first() {
                        s.push(*c);
                        p = &p[1..];
                    }

                    while let Some(c @ '0'..='9') = p.first() {
                        s.push(*c);
                        p = &p[1..];
                    }
                }

                let n = s.parse().unwrap();
                tokens.push(Token::Number(n));
            }
            'f' => {
                if let Some(['f', 'a', 'l', 's', 'e']) = &p.get(0..5) {
                    tokens.push(Token::False);
                    p = &p[5..];
                } else {
                    return Err(Error);
                }
            }
            'n' => {
                if let Some(['n', 'u', 'l', 'l']) = &p.get(0..4) {
                    tokens.push(Token::Null);
                    p = &p[4..];
                } else {
                    return Err(Error);
                }
            }
            't' => {
                if let Some(['t', 'r', 'u', 'e']) = &p.get(0..4) {
                    tokens.push(Token::True);
                    p = &p[4..];
                } else {
                    return Err(Error);
                }
            }
            _ => {
                return Err(Error);
            }
        }
    }

    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(tokenize("["), Ok(vec![Token::BeginArray]));
        assert_eq!(tokenize("{"), Ok(vec![Token::BeginObject]));
        assert_eq!(tokenize("]"), Ok(vec![Token::EndArray]));
        assert_eq!(tokenize("}"), Ok(vec![Token::EndObject]));
        assert_eq!(tokenize(":"), Ok(vec![Token::NameSeparator]));
        assert_eq!(tokenize(","), Ok(vec![Token::ValueSeparator]));

        assert_eq!(tokenize("false"), Ok(vec![Token::False]));
        assert_eq!(tokenize("null"), Ok(vec![Token::Null]));
        assert_eq!(tokenize("true"), Ok(vec![Token::True]));
        assert_eq!(tokenize("12345"), Ok(vec![Token::Number(12345f64)]));
        assert_eq!(tokenize("12345e123"), Ok(vec![Token::Number(12345e123f64)]));
        assert_eq!(
            tokenize("12345e-123"),
            Ok(vec![Token::Number(12345e-123f64)])
        );
        assert_eq!(tokenize("123.45"), Ok(vec![Token::Number(123.45f64)]));
        assert_eq!(
            tokenize("123.45e123"),
            Ok(vec![Token::Number(123.45e123f64)])
        );
        assert_eq!(
            tokenize("123.45e-123"),
            Ok(vec![Token::Number(123.45e-123f64)])
        );
        assert_eq!(tokenize("-12345"), Ok(vec![Token::Number(-12345f64)]));
        assert_eq!(
            tokenize("-12345e123"),
            Ok(vec![Token::Number(-12345e123f64)])
        );
        assert_eq!(
            tokenize("-12345e-123"),
            Ok(vec![Token::Number(-12345e-123f64)])
        );
        assert_eq!(tokenize("-123.45"), Ok(vec![Token::Number(-123.45f64)]));
        assert_eq!(
            tokenize("-123.45e123"),
            Ok(vec![Token::Number(-123.45e123f64)])
        );
        assert_eq!(
            tokenize("-123.45e-123"),
            Ok(vec![Token::Number(-123.45e-123f64)])
        );
        assert_eq!(
            tokenize("\"string\""),
            Ok(vec![Token::String("string".to_string())])
        );

        assert_eq!(tokenize("[]"), Ok(vec![Token::BeginArray, Token::EndArray]));
        assert_eq!(
            tokenize(
                r#"
                [
                    false,
                    null,
                    true,
                    12345,
                    "string",
                    [],
                    {}
                ]
                "#
            ),
            Ok(vec![
                Token::BeginArray,
                Token::False,
                Token::ValueSeparator,
                Token::Null,
                Token::ValueSeparator,
                Token::True,
                Token::ValueSeparator,
                Token::Number(12345f64),
                Token::ValueSeparator,
                Token::String("string".to_string()),
                Token::ValueSeparator,
                Token::BeginArray,
                Token::EndArray,
                Token::ValueSeparator,
                Token::BeginObject,
                Token::EndObject,
                Token::EndArray,
            ])
        );

        assert_eq!(
            tokenize("{}"),
            Ok(vec![Token::BeginObject, Token::EndObject])
        );
        assert_eq!(
            tokenize(
                r#"
                {
                    "key": "value"
                }
                "#
            ),
            Ok(vec![
                Token::BeginObject,
                Token::String("key".to_string()),
                Token::NameSeparator,
                Token::String("value".to_string()),
                Token::EndObject
            ])
        );
        assert_eq!(
            tokenize(
                r#"
                {
                    "key0": false,
                    "key1": null,
                    "key2": true,
                    "key3": 12345,
                    "key4": "string",
                    "key5": [],
                    "key6": {}
                }
                "#
            ),
            Ok(vec![
                Token::BeginObject,
                Token::String("key0".to_string()),
                Token::NameSeparator,
                Token::False,
                Token::ValueSeparator,
                Token::String("key1".to_string()),
                Token::NameSeparator,
                Token::Null,
                Token::ValueSeparator,
                Token::String("key2".to_string()),
                Token::NameSeparator,
                Token::True,
                Token::ValueSeparator,
                Token::String("key3".to_string()),
                Token::NameSeparator,
                Token::Number(12345f64),
                Token::ValueSeparator,
                Token::String("key4".to_string()),
                Token::NameSeparator,
                Token::String("string".to_string()),
                Token::ValueSeparator,
                Token::String("key5".to_string()),
                Token::NameSeparator,
                Token::BeginArray,
                Token::EndArray,
                Token::ValueSeparator,
                Token::String("key6".to_string()),
                Token::NameSeparator,
                Token::BeginObject,
                Token::EndObject,
                Token::EndObject
            ])
        );
    }
}
