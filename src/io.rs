//! Stdout move emission for the engine protocol.

use std::io::{self, Write};

use crate::model::format_move;

pub fn write_move<W: Write>(writer: &mut W, x: i32, y: i32) -> io::Result<()> {
    writer.write_all(format_move(x, y).as_bytes())?;
    writer.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_move_emits_space_separated_coordinates_with_newline() {
        let mut out = Vec::new();
        write_move(&mut out, 7, 2).expect("write should succeed");
        assert_eq!(out, b"7 2\n");
    }

    #[test]
    fn write_move_emits_fallback_coordinates() {
        let mut out = Vec::new();
        write_move(&mut out, 0, 0).expect("write should succeed");
        assert_eq!(out, b"0 0\n");
    }
}
