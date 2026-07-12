//! Move selection heuristics.

use crate::model::{Anfield, Piece, PlayerId};
use crate::placement::{
    apply_placement, count_foe_valid_placements, count_new_territory, count_voronoi_empty_for_foe,
    count_voronoi_empty_for_own, horizontal_span_own, iter_valid_placements,
    min_distance_new_cells_to_foe,
};

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

const SPAWN_COLUMN: i32 = 9;

pub fn choose_move(anfield: &Anfield, piece: &Piece, player: PlayerId) -> (i32, i32) {
    let mut placements: Vec<(i32, i32)> = iter_valid_placements(anfield, piece, player).collect();
    if placements.is_empty() {
        return FALLBACK_MOVE;
    }

    for &(x, y) in &placements {
        let after = apply_placement(anfield, piece, x, y);
        if count_foe_valid_placements(&after, piece) == 0 {
            return (x, y);
        }
    }

    placements
        .sort_by(|&(x2, y2), &(x1, y1)| cmp_placement(anfield, piece, player, x1, y1, x2, y2));

    placements[0]
}

fn cmp_placement(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> std::cmp::Ordering {
    let g1 = count_new_territory(anfield, piece, x1, y1);
    let g2 = count_new_territory(anfield, piece, x2, y2);

    g1.cmp(&g2)
        .then_with(|| spawn_column_bias(x1).cmp(&spawn_column_bias(x2)))
        .then_with(|| {
            voronoi_delta(anfield, piece, x1, y1).cmp(&voronoi_delta(anfield, piece, x2, y2))
        })
        .then_with(|| {
            foe_voronoi_delta(anfield, piece, x1, y1)
                .cmp(&foe_voronoi_delta(anfield, piece, x2, y2))
        })
        .then_with(|| {
            advance_bias(anfield, piece, player, x1, y1)
                .cmp(&advance_bias(anfield, piece, player, x2, y2))
        })
        .then_with(|| {
            min_distance_new_cells_to_foe(anfield, piece, x2, y2)
                .cmp(&min_distance_new_cells_to_foe(anfield, piece, x1, y1))
        })
        .then_with(|| {
            horizontal_span_own(&apply_placement(anfield, piece, x1, y1)).cmp(&horizontal_span_own(
                &apply_placement(anfield, piece, x2, y2),
            ))
        })
        .then_with(|| (x2, y2).cmp(&(x1, y1)))
}

fn spawn_column_bias(x: i32) -> i64 {
    -((x - SPAWN_COLUMN).abs() as i64)
}

fn advance_bias(anfield: &Anfield, piece: &Piece, player: PlayerId, x: i32, y: i32) -> i64 {
    let mut score = 0i64;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ay = y + py as i32;
            let idx = ay as usize * anfield.width + (x + px as i32) as usize;
            if anfield.cells[idx] != crate::model::Cell::Empty {
                continue;
            }
            score += match player {
                PlayerId::P1 => ay as i64,
                PlayerId::P2 => anfield.height as i64 - 1 - ay as i64,
            };
        }
    }
    score
}

fn voronoi_delta(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> i64 {
    let before = count_voronoi_empty_for_own(anfield) as i64;
    let after = apply_placement(anfield, piece, x, y);
    count_voronoi_empty_for_own(&after) as i64 - before
}

fn foe_voronoi_delta(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> i64 {
    let before = count_voronoi_empty_for_foe(anfield) as i64;
    let after = apply_placement(anfield, piece, x, y);
    before - count_voronoi_empty_for_foe(&after) as i64
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
}
