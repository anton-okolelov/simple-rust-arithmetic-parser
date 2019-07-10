use crate::scanner::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedChar(u8),
    UnexpectedToken(Token),
}
