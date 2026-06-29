//! Engine stdin protocol parsing (implemented in T10).

use crate::model::{Anfield, Piece, PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub player: PlayerId,
    pub anfield: Anfield,
    pub piece: Piece,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidExecLine,
    InvalidAnfieldHeader,
    InvalidPieceHeader,
    UnexpectedEof,
}

pub fn parse_exec_line(line: &str) -> Result<PlayerId, ParseError> {
    let _ = line;
    Err(ParseError::InvalidExecLine)
}

pub fn parse_turn(_input: &str) -> Result<Turn, ParseError> {
    Err(ParseError::UnexpectedEof)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_turn_stub_returns_error_until_t10() {
        assert_eq!(parse_turn(""), Err(ParseError::UnexpectedEof));
    }
}
