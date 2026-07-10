//! Engine stdin protocol parsing.

use crate::model::{Anfield, Cell, Piece, PlayerId};

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
    let line = line.trim();
    const PREFIX: &str = "$$$ exec p";
    if !line.starts_with(PREFIX) {
        return Err(ParseError::InvalidExecLine);
    }

    let rest = &line[PREFIX.len()..];
    let digit_end = rest
        .find(|c: char| !c.is_ascii_digit())
        .ok_or(ParseError::InvalidExecLine)?;
    if digit_end == 0 {
        return Err(ParseError::InvalidExecLine);
    }

    let num: u8 = rest[..digit_end]
        .parse()
        .map_err(|_| ParseError::InvalidExecLine)?;
    let player = PlayerId::from_exec_number(num).ok_or(ParseError::InvalidExecLine)?;

    let after = rest[digit_end..].trim_start();
    if !after.starts_with(':') || !after.contains('[') || !after.ends_with(']') {
        return Err(ParseError::InvalidExecLine);
    }

    Ok(player)
}

pub fn parse_turn(input: &str) -> Result<Turn, ParseError> {
    let lines = non_empty_lines(input);
    if lines.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }

    let player = parse_exec_line(lines[0])?;
    let (anfield, piece) = parse_board_and_piece(&lines[1..], player)?;
    Ok(Turn {
        player,
        anfield,
        piece,
    })
}

pub fn parse_turn_continuation(input: &str, player: PlayerId) -> Result<Turn, ParseError> {
    let lines = non_empty_lines(input);
    if lines.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }

    let (anfield, piece) = parse_board_and_piece(&lines, player)?;
    Ok(Turn {
        player,
        anfield,
        piece,
    })
}

fn non_empty_lines(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim_end)
        .filter(|line| !line.is_empty())
        .collect()
}

fn parse_board_and_piece(lines: &[&str], player: PlayerId) -> Result<(Anfield, Piece), ParseError> {
    if lines.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }

    let (width, height) = parse_anfield_header(lines[0])?;
    let mut idx = 1usize;

    if idx < lines.len() && is_column_header(lines[idx]) {
        idx += 1;
    }

    let mut cells = Vec::with_capacity(width * height);
    for _ in 0..height {
        let line = lines.get(idx).ok_or(ParseError::UnexpectedEof)?;
        let row = parse_anfield_row(line, width)?;
        cells.extend(row.into_iter().map(|ch| classify_cell(ch, player)));
        idx += 1;
    }

    let anfield = Anfield {
        width,
        height,
        cells,
    };

    let piece_line = lines.get(idx).ok_or(ParseError::UnexpectedEof)?;
    let (piece_width, piece_height) = parse_piece_header(piece_line)?;
    idx += 1;

    let mut mask = Vec::with_capacity(piece_width * piece_height);
    for _ in 0..piece_height {
        let line = lines.get(idx).ok_or(ParseError::UnexpectedEof)?;
        let row = parse_piece_row(line, piece_width)?;
        mask.extend(row.into_iter().map(|ch| ch != '.'));
        idx += 1;
    }

    let piece = Piece {
        width: piece_width,
        height: piece_height,
        mask,
    };

    Ok((anfield, piece))
}

fn parse_anfield_header(line: &str) -> Result<(usize, usize), ParseError> {
    let line = line.trim();
    let rest = line
        .strip_prefix("Anfield ")
        .ok_or(ParseError::InvalidAnfieldHeader)?;
    let (dims, _) = rest
        .split_once(':')
        .ok_or(ParseError::InvalidAnfieldHeader)?;
    let mut parts = dims.split_whitespace();
    let width: usize = parts
        .next()
        .ok_or(ParseError::InvalidAnfieldHeader)?
        .parse()
        .map_err(|_| ParseError::InvalidAnfieldHeader)?;
    let height: usize = parts
        .next()
        .ok_or(ParseError::InvalidAnfieldHeader)?
        .parse()
        .map_err(|_| ParseError::InvalidAnfieldHeader)?;
    if parts.next().is_some() || width == 0 || height == 0 {
        return Err(ParseError::InvalidAnfieldHeader);
    }
    Ok((width, height))
}

fn is_column_header(line: &str) -> bool {
    line.starts_with(' ') || line.starts_with('\t')
}

fn parse_anfield_row(line: &str, width: usize) -> Result<Vec<char>, ParseError> {
    let line = line.trim_end();
    let (prefix, content) = line
        .split_once(' ')
        .ok_or(ParseError::InvalidAnfieldHeader)?;
    if prefix.len() != 3 || !prefix.chars().all(|c| c.is_ascii_digit()) {
        return Err(ParseError::InvalidAnfieldHeader);
    }
    if content.chars().count() != width {
        return Err(ParseError::InvalidAnfieldHeader);
    }
    Ok(content.chars().collect())
}

fn parse_piece_header(line: &str) -> Result<(usize, usize), ParseError> {
    let line = line.trim();
    let rest = line
        .strip_prefix("Piece ")
        .ok_or(ParseError::InvalidPieceHeader)?;
    let (dims, _) = rest.split_once(':').ok_or(ParseError::InvalidPieceHeader)?;
    let mut parts = dims.split_whitespace();
    let width: usize = parts
        .next()
        .ok_or(ParseError::InvalidPieceHeader)?
        .parse()
        .map_err(|_| ParseError::InvalidPieceHeader)?;
    let height: usize = parts
        .next()
        .ok_or(ParseError::InvalidPieceHeader)?
        .parse()
        .map_err(|_| ParseError::InvalidPieceHeader)?;
    if parts.next().is_some() || width == 0 || height == 0 {
        return Err(ParseError::InvalidPieceHeader);
    }
    Ok((width, height))
}

fn parse_piece_row(line: &str, width: usize) -> Result<Vec<char>, ParseError> {
    let line = line.trim_end();
    if line.chars().count() != width {
        return Err(ParseError::InvalidPieceHeader);
    }
    Ok(line.chars().collect())
}

fn classify_cell(ch: char, player: PlayerId) -> Cell {
    match (player, ch) {
        (_, '.') => Cell::Empty,
        (PlayerId::P1, '@') => Cell::Own,
        (PlayerId::P1, 'a') => Cell::OwnLast,
        (PlayerId::P1, '$') => Cell::Foe,
        (PlayerId::P1, 's') => Cell::FoeLast,
        (PlayerId::P2, '$') => Cell::Own,
        (PlayerId::P2, 's') => Cell::OwnLast,
        (PlayerId::P2, '@') => Cell::Foe,
        (PlayerId::P2, 'a') => Cell::FoeLast,
        _ => Cell::Foe,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Cell;

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

    const BRIEF_PIECES: &str = "\
Anfield 3 2:
000 ...
001 .@.
Piece 2 2:
.#
#.";

    #[test]
    fn parse_exec_line_accepts_brief_format() {
        assert_eq!(
            parse_exec_line("$$$ exec p1 : [robots/bender]"),
            Ok(PlayerId::P1)
        );
        assert_eq!(
            parse_exec_line("$$$ exec p2 : [/filler/solution/filler]"),
            Ok(PlayerId::P2)
        );
    }

    #[test]
    fn parse_exec_line_rejects_invalid_lines() {
        assert_eq!(
            parse_exec_line("Anfield 20 15:"),
            Err(ParseError::InvalidExecLine)
        );
        assert_eq!(
            parse_exec_line("$$$ exec p9 : [robots/bender]"),
            Err(ParseError::InvalidExecLine)
        );
    }

    #[test]
    fn parse_turn_reads_brief_fixture_for_player_one() {
        let turn = parse_turn(BRIEF_TURN_P1).expect("brief fixture should parse");

        assert_eq!(turn.player, PlayerId::P1);
        assert_eq!(turn.anfield.width, 20);
        assert_eq!(turn.anfield.height, 15);
        assert_eq!(turn.anfield.cells.len(), 20 * 15);
        assert_eq!(turn.anfield.cells[2 * 20 + 9], Cell::Own);
        assert_eq!(turn.anfield.cells[12 * 20 + 9], Cell::Foe);

        assert_eq!(turn.piece.width, 4);
        assert_eq!(turn.piece.height, 1);
        assert_eq!(turn.piece.filled_count(), 2);
        assert_eq!(turn.piece.mask, vec![false, true, true, false]);
    }

    #[test]
    fn parse_turn_classifies_symbols_for_player_two() {
        let input = "\
$$$ exec p2 : [robots/wall_e]
Anfield 3 3:
000 @a.
001 .$.
002 ...
Piece 1 1:
#";

        let turn = parse_turn(input).expect("p2 fixture should parse");
        assert_eq!(turn.player, PlayerId::P2);

        let cells = &turn.anfield.cells;
        assert_eq!(cells[0], Cell::Foe);
        assert_eq!(cells[1], Cell::FoeLast);
        assert_eq!(cells[2], Cell::Empty);
        assert_eq!(cells[3], Cell::Empty);
        assert_eq!(cells[4], Cell::Own);
        assert_eq!(cells[5], Cell::Empty);
        assert_eq!(turn.piece.mask, vec![true]);
    }

    #[test]
    fn parse_turn_reads_multi_row_piece_mask() {
        let input = format!("$$$ exec p1 : [robots/bender]\n{BRIEF_PIECES}");
        let turn = parse_turn(&input).expect("piece fixture should parse");

        assert_eq!(turn.anfield.width, 3);
        assert_eq!(turn.anfield.height, 2);
        assert_eq!(turn.piece.width, 2);
        assert_eq!(turn.piece.height, 2);
        assert_eq!(turn.piece.mask, vec![false, true, true, false]);
        assert_eq!(turn.piece.filled_count(), 2);
    }

    #[test]
    fn parse_turn_rejects_empty_input() {
        assert_eq!(parse_turn(""), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn parse_turn_rejects_bad_anfield_header() {
        let input = "\
$$$ exec p1 : [robots/bender]
Board 20 15:
Piece 1 1:
#";
        assert_eq!(parse_turn(input), Err(ParseError::InvalidAnfieldHeader));
    }

    #[test]
    fn parse_turn_continuation_reads_anfield_and_piece_only() {
        let input = "\
Anfield 3 2:
000 ...
001 .@.
Piece 2 2:
.#
#.";

        let turn = parse_turn_continuation(input, PlayerId::P2).expect("continuation should parse");
        assert_eq!(turn.player, PlayerId::P2);
        assert_eq!(turn.anfield.width, 3);
        assert_eq!(turn.anfield.height, 2);
        assert_eq!(turn.piece.mask, vec![false, true, true, false]);
    }
}
