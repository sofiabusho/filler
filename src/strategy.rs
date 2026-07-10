//! Move selection heuristics (scoring tuned in T13).

use crate::model::{Anfield, Piece, PlayerId};
use crate::placement::iter_valid_placements;

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

pub fn choose_move(anfield: &Anfield, piece: &Piece, player: PlayerId) -> (i32, i32) {
    iter_valid_placements(anfield, piece, player)
        .next()
        .unwrap_or(FALLBACK_MOVE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Anfield, Cell, Piece};

    #[test]
    fn choose_move_returns_fallback_when_no_valid_placement() {
        let anfield = Anfield {
            width: 1,
            height: 1,
            cells: vec![Cell::Empty],
        };
        let piece = Piece {
            width: 1,
            height: 1,
            mask: vec![true],
        };
        assert_eq!(choose_move(&anfield, &piece, PlayerId::P1), FALLBACK_MOVE);
    }

    #[test]
    fn choose_move_returns_first_valid_placement() {
        let anfield = Anfield {
            width: 3,
            height: 3,
            cells: vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Own,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        };
        let piece = Piece {
            width: 1,
            height: 1,
            mask: vec![true],
        };
        assert_eq!(choose_move(&anfield, &piece, PlayerId::P1), (1, 1));
    }
}
