//! Move selection heuristics.

use crate::model::{Anfield, Piece, PlayerId};
use crate::placement::{count_new_territory, iter_valid_placements};

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

pub fn choose_move(anfield: &Anfield, piece: &Piece, player: PlayerId) -> (i32, i32) {
    iter_valid_placements(anfield, piece, player)
        .max_by(|&(x1, y1), &(x2, y2)| {
            let gain1 = count_new_territory(anfield, piece, x1, y1);
            let gain2 = count_new_territory(anfield, piece, x2, y2);
            gain1.cmp(&gain2).then_with(|| (x2, y2).cmp(&(x1, y1)))
        })
        .unwrap_or(FALLBACK_MOVE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Anfield, Cell, Piece};

    fn anfield_from_rows(rows: &[&str]) -> Anfield {
        let height = rows.len();
        let width = rows[0].len();
        Anfield {
            width,
            height,
            cells: rows
                .iter()
                .flat_map(|row| {
                    row.chars().map(|ch| match ch {
                        '.' => Cell::Empty,
                        '@' => Cell::Own,
                        'a' => Cell::OwnLast,
                        '$' => Cell::Foe,
                        's' => Cell::FoeLast,
                        _ => Cell::Foe,
                    })
                })
                .collect(),
        }
    }

    fn piece_from_rows(rows: &[&str]) -> Piece {
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
    fn choose_move_maximizes_new_territory() {
        let anfield = anfield_from_rows(&["....", ".@..", "...."]);
        let wide = piece_from_rows(&["###"]);
        let tall = piece_from_rows(&["#", "#"]);

        assert_eq!(choose_move(&anfield, &wide, PlayerId::P1), (0, 1));
        assert!(
            count_new_territory(&anfield, &wide, 0, 1) > count_new_territory(&anfield, &tall, 1, 0)
        );
    }

    #[test]
    fn choose_move_prefers_smaller_coordinates_on_equal_gain() {
        let anfield = anfield_from_rows(&[".@."]);
        let line = piece_from_rows(&["##"]);

        assert_eq!(choose_move(&anfield, &line, PlayerId::P1), (0, 0));
    }
}
