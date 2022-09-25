use crate::tokenize::Token;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    UnexpectedEnd,
    UnexpectedChar(char),
    UnexpectedToken(&'a Token<'a>),
}
