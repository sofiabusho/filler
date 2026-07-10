//! Stdin game loop wiring parse, strategy, and output.

use std::io::{self, BufRead, Write};

use crate::io::write_move;
use crate::model::PlayerId;
use crate::parse::{self, ParseError, Turn};
use crate::strategy;

pub fn run_game<R: BufRead, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<()> {
    let mut player: Option<PlayerId> = None;
    let mut pending_lines = Vec::new();

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        pending_lines.push(line);
        if let Some(turn) = try_parse_pending_turn(&pending_lines, player)? {
            player = Some(turn.player);
            let (x, y) = strategy::choose_move(&turn.anfield, &turn.piece, turn.player);
            write_move(writer, x, y)?;
            pending_lines.clear();
        }
    }

    Ok(())
}

fn try_parse_pending_turn(lines: &[String], player: Option<PlayerId>) -> io::Result<Option<Turn>> {
    let input = lines.concat();
    let turn = match player {
        None => match parse::parse_turn(&input) {
            Ok(turn) => turn,
            Err(ParseError::UnexpectedEof) => return Ok(None),
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{err:?}"),
                ))
            }
        },
        Some(player) => match parse::parse_turn_continuation(&input, player) {
            Ok(turn) => turn,
            Err(ParseError::UnexpectedEof) => return Ok(None),
            Err(err) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("{err:?}"),
                ))
            }
        },
    };

    Ok(Some(turn))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const BRIEF_TURN_P1: &str = "\
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.";

    const NO_VALID_TURN: &str = "\
$$$ exec p1 : [robots/bender]
Anfield 3 3:
000 ...
001 ...
002 ...
Piece 1 1:
#";

    #[test]
    fn run_game_emits_valid_move_for_brief_fixture() {
        let mut input = Cursor::new(BRIEF_TURN_P1);
        let mut output = Vec::new();

        run_game(&mut input, &mut output).expect("game loop should succeed");
        assert_eq!(output, b"7 2\n");
    }

    #[test]
    fn run_game_emits_fallback_when_no_valid_placement() {
        let mut input = Cursor::new(NO_VALID_TURN);
        let mut output = Vec::new();

        run_game(&mut input, &mut output).expect("game loop should succeed");
        assert_eq!(output, b"0 0\n");
    }

    #[test]
    fn run_game_handles_multiple_turns_after_exec_line() {
        let input =
            format!("{BRIEF_TURN_P1}\nAnfield 3 3:\n000 .@.\n001 ...\n002 ...\nPiece 1 1:\n#\n");
        let mut reader = Cursor::new(input);
        let mut output = Vec::new();

        run_game(&mut reader, &mut output).expect("multi-turn loop should succeed");
        assert_eq!(output, b"7 2\n1 0\n");
    }
}
