//! Placement validation.

use crate::model::{Anfield, Cell, Piece, PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementError {
    OutOfBounds,
    OpponentOverlap,
    WrongOverlapCount { got: u32 },
}

pub fn validate_placement(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
    x: i32,
    y: i32,
) -> Result<(), PlacementError> {
    if !piece_fits_in_bounds(anfield, piece, x, y) {
        return Err(PlacementError::OutOfBounds);
    }

    let own_overlaps = count_own_overlaps(anfield, piece, player, x, y);
    if has_opponent_overlap(anfield, piece, x, y) {
        return Err(PlacementError::OpponentOverlap);
    }

    if own_overlaps != 1 {
        return Err(PlacementError::WrongOverlapCount { got: own_overlaps });
    }

    Ok(())
}

pub fn count_own_overlaps(
    anfield: &Anfield,
    piece: &Piece,
    _player: PlayerId,
    x: i32,
    y: i32,
) -> u32 {
    if !piece_fits_in_bounds(anfield, piece, x, y) {
        return 0;
    }

    let mut count = 0u32;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let cell = anfield_cell(anfield, x + px as i32, y + py as i32);
            if matches!(cell, Cell::Own | Cell::OwnLast) {
                count += 1;
            }
        }
    }
    count
}

pub fn iter_valid_placements<'a>(
    anfield: &'a Anfield,
    piece: &'a Piece,
    player: PlayerId,
) -> impl Iterator<Item = (i32, i32)> + 'a {
    let max_x = anfield.width.saturating_sub(piece.width) as i32;
    let max_y = anfield.height.saturating_sub(piece.height) as i32;

    (0..=max_x).flat_map(move |x| {
        (0..=max_y).filter_map(move |y| {
            validate_placement(anfield, piece, player, x, y)
                .ok()
                .map(|()| (x, y))
        })
    })
}

fn piece_fits_in_bounds(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let ay = y + py as i32;
            if ax < 0 || ay < 0 || ax >= anfield.width as i32 || ay >= anfield.height as i32 {
                return false;
            }
        }
    }
    true
}

fn has_opponent_overlap(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> bool {
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let cell = anfield_cell(anfield, x + px as i32, y + py as i32);
            if matches!(cell, Cell::Foe | Cell::FoeLast) {
                return true;
            }
        }
    }
    false
}

fn anfield_cell(anfield: &Anfield, x: i32, y: i32) -> Cell {
    anfield.cells[y as usize * anfield.width + x as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Anfield, Cell, Piece};

    fn anfield(cells: &[&str]) -> Anfield {
        let height = cells.len();
        let width = cells[0].len();
        Anfield {
            width,
            height,
            cells: cells
                .iter()
                .flat_map(|row| row.chars().map(cell_from_char))
                .collect(),
        }
    }

    fn cell_from_char(ch: char) -> Cell {
        match ch {
            '.' => Cell::Empty,
            '@' => Cell::Own,
            'a' => Cell::OwnLast,
            '$' => Cell::Foe,
            's' => Cell::FoeLast,
            _ => Cell::Foe,
        }
    }

    fn piece(rows: &[&str]) -> Piece {
        let height = rows.len();
        let width = rows[0].len();
        Piece {
            width,
            height,
            mask: rows
                .iter()
                .flat_map(|row| row.chars().map(|ch| ch != '.'))
                .collect(),
        }
    }

    #[test]
    fn validate_accepts_exactly_one_own_overlap() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#"]);

        assert_eq!(validate_placement(&board, &tet, PlayerId::P1, 1, 1), Ok(()));
    }

    #[test]
    fn validate_rejects_zero_own_overlaps() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#"]);

        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 0, 0),
            Err(PlacementError::WrongOverlapCount { got: 0 })
        );
    }

    #[test]
    fn validate_rejects_two_or_more_own_overlaps() {
        let board = anfield(&["...", ".@.", ".@.", "..."]);
        let tet = piece(&["#", "#"]);

        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 1, 1),
            Err(PlacementError::WrongOverlapCount { got: 2 })
        );
    }

    #[test]
    fn validate_rejects_opponent_overlap() {
        let board = anfield(&["...", ".$.", ".@.", "..."]);
        let tet = piece(&["#", "#"]);

        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 1, 0),
            Err(PlacementError::OpponentOverlap)
        );
    }

    #[test]
    fn validate_rejects_negative_anchor() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#"]);

        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, -1, 1),
            Err(PlacementError::OutOfBounds)
        );
        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 1, -1),
            Err(PlacementError::OutOfBounds)
        );
    }

    #[test]
    fn validate_rejects_piece_extending_past_board_edges() {
        let board = anfield(&["..", ".@"]);
        let tet = piece(&["##", "##"]);

        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 1, 0),
            Err(PlacementError::OutOfBounds)
        );
        assert_eq!(
            validate_placement(&board, &tet, PlayerId::P1, 0, 1),
            Err(PlacementError::OutOfBounds)
        );
    }

    #[test]
    fn count_own_overlaps_returns_zero_when_out_of_bounds() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#"]);

        assert_eq!(count_own_overlaps(&board, &tet, PlayerId::P1, -1, 1), 0);
    }

    #[test]
    fn iter_valid_placements_finds_only_valid_anchors() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#", "#"]);

        let valid: Vec<_> = iter_valid_placements(&board, &tet, PlayerId::P1).collect();
        assert_eq!(valid, vec![(1, 0), (1, 1)]);
    }
}
