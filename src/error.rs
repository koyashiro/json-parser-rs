#[derive(Eq, Debug, PartialEq)]
pub enum Error {
    UnexpectedEnd,
    UnexpectedToken(char),
}
