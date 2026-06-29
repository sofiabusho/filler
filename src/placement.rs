//! Placement validation (implemented in T11).

use crate::model::{Anfield, Piece, PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlacementError {
    OutOfBounds,
    OpponentOverlap,
    WrongOverlapCount { got: u32 },
}

pub fn validate_placement(
    _anfield: &Anfield,
    _piece: &Piece,
    _player: PlayerId,
    _x: i32,
    _y: i32,
) -> Result<(), PlacementError> {
    Err(PlacementError::OutOfBounds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Anfield, Piece};

    #[test]
    fn validate_placement_stub_returns_error_until_t11() {
        let anfield = Anfield {
            width: 1,
            height: 1,
            cells: vec![crate::model::Cell::Empty],
        };
        let piece = Piece {
            width: 1,
            height: 1,
            mask: vec![true],
        };
        assert_eq!(
            validate_placement(&anfield, &piece, PlayerId::P1, 0, 0),
            Err(PlacementError::OutOfBounds)
        );
    }
}
