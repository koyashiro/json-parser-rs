use std::str;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
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
    String(&'a str),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();

    let mut p = input;

    while let Some(&c) = p.as_bytes().first() {
        match c {
            b' ' | b'\t' | b'\n' | b'\r' => {
                p = &p[1..];
            }
            b'[' => {
                tokens.push(Token::BeginArray);
                p = &p[1..];
            }
            b'{' => {
                tokens.push(Token::BeginObject);
                p = &p[1..];
            }
            b']' => {
                tokens.push(Token::EndArray);
                p = &p[1..];
            }
            b'}' => {
                tokens.push(Token::EndObject);
                p = &p[1..];
            }
            b':' => {
                tokens.push(Token::NameSeparator);
                p = &p[1..];
            }
            b',' => {
                tokens.push(Token::ValueSeparator);
                p = &p[1..];
            }
            b'"' => {
                let (cnt, s) = expect_string(p)?;
                tokens.push(Token::String(s));
                p = &p[cnt..];
            }
            b'-' | b'0'..=b'9' => {
                let (cnt, n) = expect_number(p)?;
                tokens.push(Token::Number(n));
                p = &p[cnt..];
            }
            b'f' => {
                expect_false(p)?;
                tokens.push(Token::False);
                p = &p[5..];
            }
            b'n' => {
                expect_null(p)?;
                tokens.push(Token::Null);
                p = &p[4..];
            }
            b't' => {
                expect_true(p)?;
                tokens.push(Token::True);
                p = &p[4..];
            }
            _ => {
                let c = p.chars().next().unwrap();
                return Err(Error::UnexpectedToken(c));
            }
        }
    }

    Ok(tokens)
}

fn expect_number(input: &str) -> Result<(usize, f64), Error> {
    let mut iter = input.chars().peekable();
    let mut cnt = 0;

    // minus (optional)
    if let Some('-') = iter.peek() {
        iter.next();
        cnt += 1;
    }

    // int
    while let Some('0'..='9') = iter.peek() {
        iter.next();
        cnt += 1;
    }

    // frac (optional)
    if let Some('.') = iter.peek() {
        iter.next();
        cnt += 1;

        while let Some('0'..='9') = iter.peek() {
            iter.next();
            cnt += 1;
        }
    }

    // exp (optional)
    if let Some('e') = iter.peek() {
        iter.next();
        cnt += 1;

        if let Some('+' | '-') = iter.peek() {
            iter.next();
            cnt += 1;
        }

        while let Some('0'..='9') = iter.peek() {
            iter.next();
            cnt += 1;
        }
    }

    let s = &input[..cnt];
    let n = s.parse().unwrap();

    Ok((cnt, n))
}

fn expect_string(input: &str) -> Result<(usize, &str), Error> {
    let mut iter = input.chars();
    let mut cnt = 0;
    match iter.next() {
        Some(t) if t != '"' => {
            return Err(Error::UnexpectedToken(t));
        }
        None => {
            return Err(Error::UnexpectedEnd);
        }
        _ => {
            cnt += 1;
        }
    }

    loop {
        match iter.next() {
            Some('"') => {
                let s = &input[1..cnt];
                cnt += 1;
                return Ok((cnt, s));
            }
            Some(c) => {
                cnt += c.len_utf8();
            }
            None => {
                return Err(Error::UnexpectedEnd);
            }
        }
    }
}

fn expect_null(s: &str) -> Result<(), Error> {
    let mut iter = s.chars();
    for c in ['n', 'u', 'l', 'l'] {
        match iter.next() {
            Some(t) if t != c => {
                return Err(Error::UnexpectedToken(t));
            }
            None => {
                return Err(Error::UnexpectedEnd);
            }
            _ => {}
        }
    }
    Ok(())
}

fn expect_false(s: &str) -> Result<(), Error> {
    let mut iter = s.chars();
    for c in ['f', 'a', 'l', 's', 'e'] {
        match iter.next() {
            Some(t) if t != c => {
                return Err(Error::UnexpectedToken(t));
            }
            None => {
                return Err(Error::UnexpectedEnd);
            }
            _ => {}
        }
    }
    Ok(())
}

fn expect_true(s: &str) -> Result<(), Error> {
    let mut iter = s.chars();
    for c in ['t', 'r', 'u', 'e'] {
        match iter.next() {
            Some(t) if t != c => {
                return Err(Error::UnexpectedToken(t));
            }
            None => {
                return Err(Error::UnexpectedEnd);
            }
            _ => {}
        }
    }
    Ok(())
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
        assert_eq!(tokenize("\"string\""), Ok(vec![Token::String("string")]));

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
                Token::String("string"),
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
                Token::String("key"),
                Token::NameSeparator,
                Token::String("value"),
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
                Token::String("key0"),
                Token::NameSeparator,
                Token::False,
                Token::ValueSeparator,
                Token::String("key1"),
                Token::NameSeparator,
                Token::Null,
                Token::ValueSeparator,
                Token::String("key2"),
                Token::NameSeparator,
                Token::True,
                Token::ValueSeparator,
                Token::String("key3"),
                Token::NameSeparator,
                Token::Number(12345f64),
                Token::ValueSeparator,
                Token::String("key4"),
                Token::NameSeparator,
                Token::String("string"),
                Token::ValueSeparator,
                Token::String("key5"),
                Token::NameSeparator,
                Token::BeginArray,
                Token::EndArray,
                Token::ValueSeparator,
                Token::String("key6"),
                Token::NameSeparator,
                Token::BeginObject,
                Token::EndObject,
                Token::EndObject
            ])
        );
    }
}
