//! Move selection heuristics.

use crate::model::{Anfield, Piece, PlayerId};
use crate::placement::{
    apply_placement, border_touch_bonus, count_foe_valid_placements, count_new_territory,
    count_voronoi_empty_for_foe, count_voronoi_empty_for_own, expansion_bonus,
    foe_territory_bounds, horizontal_span_own, iter_valid_placements,
    min_distance_new_cells_to_foe, own_territory_bounds, TerritoryBounds,
};

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

struct EvalContext<'a> {
    anfield: &'a Anfield,
    piece: &'a Piece,
    player: PlayerId,
    my_bounds: TerritoryBounds,
    opp_bounds: TerritoryBounds,
    center_col: i32,
}

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

    let my_bounds = own_territory_bounds(anfield);
    let eval = EvalContext {
        anfield,
        piece,
        player,
        my_bounds,
        opp_bounds: foe_territory_bounds(anfield),
        center_col: territory_center_column(my_bounds),
    };

    placements.sort_by(|&(x2, y2), &(x1, y1)| {
        placement_rank(x1, y1, &eval)
            .cmp(&placement_rank(x2, y2, &eval))
            .then_with(|| (x2, y2).cmp(&(x1, y1)))
    });

    placements[0]
}

fn territory_center_column(bounds: TerritoryBounds) -> i32 {
    (bounds.min_x + bounds.max_x) / 2
}

fn placement_rank(
    x: i32,
    y: i32,
    eval: &EvalContext<'_>,
) -> (u32, i64, i64, i64, i64, i64, i64, i64, i64, i32) {
    (
        count_new_territory(eval.anfield, eval.piece, x, y),
        expansion_bonus(eval.piece, x, y, eval.my_bounds, eval.opp_bounds),
        voronoi_delta(eval.anfield, eval.piece, x, y),
        foe_voronoi_delta(eval.anfield, eval.piece, x, y),
        territory_column_bias(x, eval.center_col),
        advance_bias(eval.anfield, eval.piece, eval.player, x, y),
        horizontal_advance_bias(eval.anfield, eval.piece, eval.player, x, y),
        border_touch_bonus(eval.anfield, eval.piece, x, y) as i64,
        -(min_distance_new_cells_to_foe(eval.anfield, eval.piece, x, y) as i64),
        horizontal_span_own(&apply_placement(eval.anfield, eval.piece, x, y)),
    )
}

fn territory_column_bias(x: i32, center_col: i32) -> i64 {
    -((x - center_col).abs() as i64)
}

fn horizontal_advance_bias(
    anfield: &Anfield,
    piece: &Piece,
    player: PlayerId,
    x: i32,
    y: i32,
) -> i64 {
    let mut score = 0i64;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let idx = (y + py as i32) as usize * anfield.width + ax as usize;
            if anfield.cells[idx] != crate::model::Cell::Empty {
                continue;
            }
            score += match player {
                PlayerId::P1 => ax as i64,
                PlayerId::P2 => anfield.width as i64 - 1 - ax as i64,
            };
        }
    }
    score
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

    #[test]
    fn map00_spawn_column_stays_centered() {
        let mut cells = vec![Cell::Empty; 20 * 15];
        cells[2 * 20 + 9] = Cell::Own;
        cells[12 * 20 + 9] = Cell::Foe;
        let anfield = Anfield {
            width: 20,
            height: 15,
            cells,
        };
        let piece = Piece {
            width: 2,
            height: 1,
            mask: vec![true, true],
        };
        assert_eq!(choose_move(&anfield, &piece, PlayerId::P1), (9, 2));
    }
}
