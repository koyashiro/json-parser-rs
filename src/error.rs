#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEnd,
    UnexpectedToken(char),
}
