//! Move selection heuristics (implemented in T13).

use crate::model::{Anfield, Piece, PlayerId};

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

pub fn choose_move(_anfield: &Anfield, _piece: &Piece, _player: PlayerId) -> (i32, i32) {
    FALLBACK_MOVE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Anfield, Cell, Piece};

    #[test]
    fn choose_move_returns_fallback_until_t13() {
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
}
