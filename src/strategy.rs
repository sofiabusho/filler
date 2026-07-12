//! Move selection heuristics.

use crate::model::{Anfield, Piece, PlayerId};
use crate::placement::{
    apply_placement, border_touch_bonus, count_foe_valid_placements, count_new_territory,
    count_reachable_empty_from_own, count_voronoi_empty_for_foe, count_voronoi_empty_for_own,
    expansion_bonus, foe_territory_bounds, horizontal_span_own, iter_valid_placements,
    iter_valid_placements_near_own, min_distance_new_cells_to_foe, own_territory_bounds,
    TerritoryBounds,
};

/// Fallback `(0, 0)` when no valid placement exists (REQ-7).
pub const FALLBACK_MOVE: (i32, i32) = (0, 0);

/// Above this cell count, use bounded search and lightweight scoring (map02).
const LARGE_MAP_CELLS: usize = 2_500;
const LARGE_MAP_CANDIDATE_CAP: usize = 64;
const LARGE_MAP_VORONOI_REFINE: usize = 16;
const INSTANT_WIN_SCAN_CAP: usize = 24;

struct EvalContext<'a> {
    anfield: &'a Anfield,
    piece: &'a Piece,
    my_bounds: TerritoryBounds,
    opp_bounds: TerritoryBounds,
    center_col: i32,
    large_map: bool,
}

pub fn choose_move(anfield: &Anfield, piece: &Piece, player: PlayerId) -> (i32, i32) {
    let large_map = anfield.width * anfield.height > LARGE_MAP_CELLS;
    let mut placements: Vec<(i32, i32)> = if large_map {
        iter_valid_placements_near_own(anfield, piece, player).collect()
    } else {
        iter_valid_placements(anfield, piece, player).collect()
    };

    if placements.is_empty() {
        return FALLBACK_MOVE;
    }

    if let Some(instant) = find_instant_win(anfield, piece, &placements) {
        return instant;
    }

    if large_map {
        placements = narrow_large_map_candidates(anfield, piece, placements);
    }

    let my_bounds = own_territory_bounds(anfield);
    let eval = EvalContext {
        anfield,
        piece,
        my_bounds,
        opp_bounds: foe_territory_bounds(anfield),
        center_col: territory_center_column(my_bounds),
        large_map,
    };

    if large_map {
        return choose_large_map_move(placements, &eval);
    }

    placements
        .into_iter()
        .max_by(|&(x1, y1), &(x2, y2)| {
            rank(x1, y1, &eval)
                .cmp(&rank(x2, y2, &eval))
                .then_with(|| (x2, y2).cmp(&(x1, y1)))
        })
        .unwrap_or(FALLBACK_MOVE)
}

fn choose_large_map_move(placements: Vec<(i32, i32)>, eval: &EvalContext<'_>) -> (i32, i32) {
    let mut ranked = placements;
    ranked.sort_by(|&(x2, y2), &(x1, y1)| {
        light_rank(x1, y1, eval)
            .cmp(&light_rank(x2, y2, eval))
            .then_with(|| (x2, y2).cmp(&(x1, y1)))
    });
    ranked.truncate(LARGE_MAP_VORONOI_REFINE);

    ranked
        .into_iter()
        .max_by(|&(x1, y1), &(x2, y2)| {
            large_map_rank(x1, y1, eval)
                .cmp(&large_map_rank(x2, y2, eval))
                .then_with(|| (x2, y2).cmp(&(x1, y1)))
        })
        .unwrap_or(FALLBACK_MOVE)
}

fn find_instant_win(
    anfield: &Anfield,
    piece: &Piece,
    placements: &[(i32, i32)],
) -> Option<(i32, i32)> {
    let mut by_gain = placements.to_vec();
    by_gain.sort_by(|&(x2, y2), &(x1, y1)| {
        count_new_territory(anfield, piece, x1, y1)
            .cmp(&count_new_territory(anfield, piece, x2, y2))
            .then_with(|| (x2, y2).cmp(&(x1, y1)))
    });
    for &(x, y) in by_gain.iter().take(INSTANT_WIN_SCAN_CAP) {
        let after = apply_placement(anfield, piece, x, y);
        if count_foe_valid_placements(&after, piece) == 0 {
            return Some((x, y));
        }
    }
    None
}

fn narrow_large_map_candidates(
    anfield: &Anfield,
    piece: &Piece,
    mut placements: Vec<(i32, i32)>,
) -> Vec<(i32, i32)> {
    placements.sort_by(|&(x2, y2), &(x1, y1)| {
        count_new_territory(anfield, piece, x1, y1)
            .cmp(&count_new_territory(anfield, piece, x2, y2))
            .then_with(|| (x2, y2).cmp(&(x1, y1)))
    });
    let max_gain = count_new_territory(anfield, piece, placements[0].0, placements[0].1);
    let gain_floor = max_gain.saturating_sub(1);
    placements.retain(|&(x, y)| count_new_territory(anfield, piece, x, y) >= gain_floor);
    placements.truncate(LARGE_MAP_CANDIDATE_CAP);
    placements
}

fn territory_center_column(bounds: TerritoryBounds) -> i32 {
    (bounds.min_x + bounds.max_x) / 2
}

fn rank(x: i32, y: i32, eval: &EvalContext<'_>) -> Rank {
    if eval.large_map {
        Rank::Light(light_rank(x, y, eval))
    } else {
        Rank::Full(full_rank(x, y, eval))
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    Light(LightRank),
    Full(FullRank),
}

type LightRank = (u32, i64, i64, i64, i64, i64);
type LargeMapRank = (u32, i64, i64, i64, i64, i64, i64, i64);
type FullRank = (u32, i64, i64, i64, i64, i64, i64, i64, i64);

fn toward_opponent_bias(x: i32, y: i32, eval: &EvalContext<'_>) -> i64 {
    let dx =
        eval.opp_bounds.min_x + eval.opp_bounds.max_x - eval.my_bounds.min_x - eval.my_bounds.max_x;
    let dy =
        eval.opp_bounds.min_y + eval.opp_bounds.max_y - eval.my_bounds.min_y - eval.my_bounds.max_y;

    let mut score = 0i64;
    for py in 0..eval.piece.height {
        for px in 0..eval.piece.width {
            if !eval.piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let ay = y + py as i32;
            let idx = ay as usize * eval.anfield.width + ax as usize;
            if eval.anfield.cells[idx] != crate::model::Cell::Empty {
                continue;
            }
            score += if dx >= 0 { ax as i64 } else { -(ax as i64) };
            score += if dy >= 0 { ay as i64 } else { -(ay as i64) };
        }
    }
    score
}

fn light_rank(x: i32, y: i32, eval: &EvalContext<'_>) -> LightRank {
    (
        count_new_territory(eval.anfield, eval.piece, x, y),
        expansion_bonus(eval.piece, x, y, eval.my_bounds, eval.opp_bounds),
        toward_opponent_bias(x, y, eval),
        -(min_distance_new_cells_to_foe(eval.anfield, eval.piece, x, y) as i64),
        reachable_own_delta(eval.anfield, eval.piece, x, y),
        border_touch_bonus(eval.anfield, eval.piece, x, y) as i64,
    )
}

fn large_map_rank(x: i32, y: i32, eval: &EvalContext<'_>) -> LargeMapRank {
    (
        count_new_territory(eval.anfield, eval.piece, x, y),
        expansion_bonus(eval.piece, x, y, eval.my_bounds, eval.opp_bounds),
        voronoi_delta(eval.anfield, eval.piece, x, y),
        foe_voronoi_delta(eval.anfield, eval.piece, x, y),
        toward_opponent_bias(x, y, eval),
        -(min_distance_new_cells_to_foe(eval.anfield, eval.piece, x, y) as i64),
        border_touch_bonus(eval.anfield, eval.piece, x, y) as i64,
        horizontal_span_own(&apply_placement(eval.anfield, eval.piece, x, y)) as i64,
    )
}

fn full_rank(x: i32, y: i32, eval: &EvalContext<'_>) -> FullRank {
    (
        count_new_territory(eval.anfield, eval.piece, x, y),
        expansion_bonus(eval.piece, x, y, eval.my_bounds, eval.opp_bounds),
        voronoi_delta(eval.anfield, eval.piece, x, y),
        foe_voronoi_delta(eval.anfield, eval.piece, x, y),
        toward_opponent_bias(x, y, eval),
        territory_column_bias(x, eval.center_col),
        border_touch_bonus(eval.anfield, eval.piece, x, y) as i64,
        -(min_distance_new_cells_to_foe(eval.anfield, eval.piece, x, y) as i64),
        horizontal_span_own(&apply_placement(eval.anfield, eval.piece, x, y)) as i64,
    )
}

fn territory_column_bias(x: i32, center_col: i32) -> i64 {
    -((x - center_col).abs() as i64)
}

fn reachable_own_delta(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> i64 {
    let before = count_reachable_empty_from_own(anfield) as i64;
    let after = apply_placement(anfield, piece, x, y);
    count_reachable_empty_from_own(&after) as i64 - before
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
