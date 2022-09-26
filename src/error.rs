use std::{
    error::Error as StdError,
    fmt::{Display, Error as FmtError, Formatter},
};

#[derive(Eq, Debug, PartialEq)]
pub enum Error {
    UnexpectedEnd,
    UnexpectedChar(char),
    UnexpectedToken(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            Self::UnexpectedEnd => write!(f, "unexpected end"),
            Self::UnexpectedChar(c) => write!(f, "unexpected token '{c}'"),
            Self::UnexpectedToken(t) => write!(f, "unexpected token '{t}'"),
        }
    }
}

impl StdError for Error {}

#[cfg(test)]
mod tests {
    use crate::tokenize::Token;

    use super::*;

    #[test]
    fn display_test() {
        assert_eq!(&Error::UnexpectedEnd.to_string(), "unexpected end");
        assert_eq!(
            &Error::UnexpectedChar('a').to_string(),
            "unexpected token 'a'"
        );
        assert_eq!(
            &Error::UnexpectedToken(Token::BeginArray.to_string()).to_string(),
            "unexpected token '['"
        );
    }
}
