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

pub fn count_new_territory(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> u32 {
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
            if cell == Cell::Empty {
                count += 1;
            }
        }
    }
    count
}

pub fn apply_placement(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> Anfield {
    let mut cells = anfield.cells.clone();
    for cell in &mut cells {
        match cell {
            Cell::OwnLast => *cell = Cell::Own,
            Cell::FoeLast => *cell = Cell::Foe,
            _ => {}
        }
    }

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let idx = (y + py as i32) as usize * anfield.width + (x + px as i32) as usize;
            cells[idx] = Cell::OwnLast;
        }
    }

    Anfield {
        width: anfield.width,
        height: anfield.height,
        cells,
    }
}

pub fn min_distance_new_cells_to_foe(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> i32 {
    let mut best = i32::MAX;
    let mut found = false;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let ay = y + py as i32;
            if anfield_cell(anfield, ax, ay) != Cell::Empty {
                continue;
            }
            found = true;
            for fy in 0..anfield.height {
                for fx in 0..anfield.width {
                    if !matches!(
                        anfield.cells[fy * anfield.width + fx],
                        Cell::Foe | Cell::FoeLast
                    ) {
                        continue;
                    }
                    let dist = (ax - fx as i32).abs() + (ay - fy as i32);
                    best = best.min(dist);
                }
            }
        }
    }

    if found {
        best
    } else {
        0
    }
}

pub fn count_voronoi_empty_for_own(anfield: &Anfield) -> u32 {
    let size = anfield.width * anfield.height;
    let mut own_dist = vec![i32::MAX; size];
    let mut foe_dist = vec![i32::MAX; size];
    bfs_distances(
        anfield,
        |cell| matches!(cell, Cell::Own | Cell::OwnLast),
        &mut own_dist,
    );
    bfs_distances(
        anfield,
        |cell| matches!(cell, Cell::Foe | Cell::FoeLast),
        &mut foe_dist,
    );

    let mut count = 0u32;
    for idx in 0..size {
        if anfield.cells[idx] != Cell::Empty {
            continue;
        }
        if own_dist[idx] < foe_dist[idx] {
            count += 1;
        }
    }
    count
}

pub fn count_voronoi_empty_for_foe(anfield: &Anfield) -> u32 {
    let size = anfield.width * anfield.height;
    let mut own_dist = vec![i32::MAX; size];
    let mut foe_dist = vec![i32::MAX; size];
    bfs_distances(
        anfield,
        |cell| matches!(cell, Cell::Own | Cell::OwnLast),
        &mut own_dist,
    );
    bfs_distances(
        anfield,
        |cell| matches!(cell, Cell::Foe | Cell::FoeLast),
        &mut foe_dist,
    );

    let mut count = 0u32;
    for idx in 0..size {
        if anfield.cells[idx] != Cell::Empty {
            continue;
        }
        if foe_dist[idx] < own_dist[idx] {
            count += 1;
        }
    }
    count
}

pub fn border_touch_bonus(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> u32 {
    let mut count = 0u32;
    let max_x = anfield.width as i32 - 1;
    let max_y = anfield.height as i32 - 1;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let ay = y + py as i32;
            if anfield_cell(anfield, ax, ay) != Cell::Empty {
                continue;
            }
            if ax == 0 || ax == max_x || ay == 0 || ay == max_y {
                count += 1;
            }
        }
    }
    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerritoryBounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

pub fn own_territory_bounds(anfield: &Anfield) -> TerritoryBounds {
    let mut bounds = TerritoryBounds {
        min_x: i32::MAX,
        max_x: i32::MIN,
        min_y: i32::MAX,
        max_y: i32::MIN,
    };
    let mut found = false;

    for y in 0..anfield.height {
        for x in 0..anfield.width {
            if !matches!(
                anfield.cells[y * anfield.width + x],
                Cell::Own | Cell::OwnLast
            ) {
                continue;
            }
            found = true;
            bounds.min_x = bounds.min_x.min(x as i32);
            bounds.max_x = bounds.max_x.max(x as i32);
            bounds.min_y = bounds.min_y.min(y as i32);
            bounds.max_y = bounds.max_y.max(y as i32);
        }
    }

    if found {
        bounds
    } else {
        TerritoryBounds {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }
}

pub fn foe_territory_bounds(anfield: &Anfield) -> TerritoryBounds {
    let mut bounds = TerritoryBounds {
        min_x: i32::MAX,
        max_x: i32::MIN,
        min_y: i32::MAX,
        max_y: i32::MIN,
    };
    let mut found = false;

    for y in 0..anfield.height {
        for x in 0..anfield.width {
            if !matches!(
                anfield.cells[y * anfield.width + x],
                Cell::Foe | Cell::FoeLast
            ) {
                continue;
            }
            found = true;
            bounds.min_x = bounds.min_x.min(x as i32);
            bounds.max_x = bounds.max_x.max(x as i32);
            bounds.min_y = bounds.min_y.min(y as i32);
            bounds.max_y = bounds.max_y.max(y as i32);
        }
    }

    if found {
        bounds
    } else {
        TerritoryBounds {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }
}

pub fn iter_foe_valid_placements<'a>(
    anfield: &'a Anfield,
    piece: &'a Piece,
) -> impl Iterator<Item = (i32, i32)> + 'a {
    let max_x = anfield.width.saturating_sub(piece.width) as i32;
    let max_y = anfield.height.saturating_sub(piece.height) as i32;

    (0..=max_x).flat_map(move |x| {
        (0..=max_y).filter_map(move |y| {
            validate_foe_placement(anfield, piece, x, y)
                .ok()
                .map(|()| (x, y))
        })
    })
}

pub fn apply_foe_placement(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> Anfield {
    let mut cells = anfield.cells.clone();
    for cell in &mut cells {
        match cell {
            Cell::OwnLast => *cell = Cell::Own,
            Cell::FoeLast => *cell = Cell::Foe,
            _ => {}
        }
    }

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let idx = (y + py as i32) as usize * anfield.width + (x + px as i32) as usize;
            cells[idx] = Cell::FoeLast;
        }
    }

    Anfield {
        width: anfield.width,
        height: anfield.height,
        cells,
    }
}

pub fn count_foe_valid_placements(anfield: &Anfield, piece: &Piece) -> u32 {
    iter_foe_valid_placements(anfield, piece).count() as u32
}

pub fn count_foe_new_territory(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> u32 {
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
            if cell == Cell::Empty {
                count += 1;
            }
        }
    }
    count
}

pub fn board_eval(anfield: &Anfield) -> i64 {
    count_voronoi_empty_for_own(anfield) as i64 - count_voronoi_empty_for_foe(anfield) as i64
}

pub fn external_empty_cells(anfield: &Anfield) -> Vec<bool> {
    use std::collections::VecDeque;

    let size = anfield.width * anfield.height;
    let mut external = vec![false; size];
    let mut queue = VecDeque::new();

    for x in 0..anfield.width {
        for &y in &[0, anfield.height - 1] {
            let idx = y * anfield.width + x;
            if anfield.cells[idx] == Cell::Empty && !external[idx] {
                external[idx] = true;
                queue.push_back((x, y));
            }
        }
    }
    for y in 0..anfield.height {
        for &x in &[0, anfield.width - 1] {
            let idx = y * anfield.width + x;
            if anfield.cells[idx] == Cell::Empty && !external[idx] {
                external[idx] = true;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in neighbors(x, y, anfield.width, anfield.height) {
            let idx = ny * anfield.width + nx;
            if external[idx] || anfield.cells[idx] != Cell::Empty {
                continue;
            }
            external[idx] = true;
            queue.push_back((nx, ny));
        }
    }

    external
}

pub fn foe_frontier_mask(anfield: &Anfield, external: &[bool]) -> Vec<bool> {
    let size = anfield.width * anfield.height;
    let mut frontier = vec![false; size];

    for y in 0..anfield.height {
        for x in 0..anfield.width {
            let idx = y * anfield.width + x;
            if !matches!(anfield.cells[idx], Cell::Foe | Cell::FoeLast) {
                continue;
            }
            let mut exposed = false;
            for (nx, ny) in neighbors(x, y, anfield.width, anfield.height) {
                let nidx = ny * anfield.width + nx;
                if anfield.cells[nidx] == Cell::Empty && external[nidx] {
                    exposed = true;
                    break;
                }
            }
            if exposed {
                frontier[idx] = true;
            }
        }
    }

    frontier
}

pub fn frontier_block_score(
    anfield: &Anfield,
    piece: &Piece,
    x: i32,
    y: i32,
    frontier: &[bool],
) -> u32 {
    let mut score = 0u32;
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let ax = x + px as i32;
            let ay = y + py as i32;
            if anfield_cell(anfield, ax, ay) != Cell::Empty {
                continue;
            }
            for (nx, ny) in neighbors(ax as usize, ay as usize, anfield.width, anfield.height) {
                if frontier[ny * anfield.width + nx] {
                    score += 1;
                }
            }
        }
    }
    score
}

pub fn expansion_bonus(
    piece: &Piece,
    x: i32,
    y: i32,
    my_bounds: TerritoryBounds,
    opp_bounds: TerritoryBounds,
) -> i64 {
    let piece_max_x = x + piece.width as i32 - 1;
    let piece_max_y = y + piece.height as i32 - 1;

    let left = my_bounds.min_x.saturating_sub(x) as i64;
    let right = piece_max_x.saturating_sub(my_bounds.max_x) as i64;
    let top = my_bounds.min_y.saturating_sub(y) as i64;
    let bottom = piece_max_y.saturating_sub(my_bounds.max_y) as i64;

    let left_w = if opp_bounds.max_x < my_bounds.min_x {
        5
    } else {
        1
    };
    let right_w = if opp_bounds.min_x > my_bounds.max_x {
        5
    } else {
        1
    };
    let top_w = if opp_bounds.max_y < my_bounds.min_y {
        5
    } else {
        1
    };
    let bottom_w = if opp_bounds.min_y > my_bounds.max_y {
        5
    } else {
        1
    };

    left * left_w + right * right_w + top * top_w + bottom * bottom_w
}

pub fn horizontal_span_own(anfield: &Anfield) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    for y in 0..anfield.height {
        for x in 0..anfield.width {
            if matches!(
                anfield.cells[y * anfield.width + x],
                Cell::Own | Cell::OwnLast
            ) {
                min_x = min_x.min(x as i32);
                max_x = max_x.max(x as i32);
            }
        }
    }

    if min_x == i32::MAX {
        0
    } else {
        max_x - min_x
    }
}

fn bfs_distances(anfield: &Anfield, is_seed: fn(Cell) -> bool, dist: &mut [i32]) {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    for y in 0..anfield.height {
        for x in 0..anfield.width {
            let idx = y * anfield.width + x;
            if is_seed(anfield.cells[idx]) {
                dist[idx] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let base = dist[y * anfield.width + x];
        for (nx, ny) in neighbors(x, y, anfield.width, anfield.height) {
            let idx = ny * anfield.width + nx;
            if anfield.cells[idx] != Cell::Empty || dist[idx] != i32::MAX {
                continue;
            }
            dist[idx] = base + 1;
            queue.push_back((nx, ny));
        }
    }
}

pub fn count_reachable_empty_from_own(anfield: &Anfield) -> u32 {
    count_reachable_empty(anfield, |cell| matches!(cell, Cell::Own | Cell::OwnLast))
}

pub fn count_reachable_empty_from_foe(anfield: &Anfield) -> u32 {
    count_reachable_empty(anfield, |cell| matches!(cell, Cell::Foe | Cell::FoeLast))
}

fn count_reachable_empty(anfield: &Anfield, is_seed: fn(Cell) -> bool) -> u32 {
    use std::collections::VecDeque;

    let size = anfield.width * anfield.height;
    let mut visited = vec![false; size];
    let mut queue = VecDeque::new();

    for y in 0..anfield.height {
        for x in 0..anfield.width {
            let idx = y * anfield.width + x;
            if is_seed(anfield.cells[idx]) {
                visited[idx] = true;
                queue.push_back((x, y));
            }
        }
    }

    let mut reachable_empty = 0u32;
    while let Some((x, y)) = queue.pop_front() {
        for (nx, ny) in neighbors(x, y, anfield.width, anfield.height) {
            let idx = ny * anfield.width + nx;
            if visited[idx] {
                continue;
            }
            if anfield.cells[idx] != Cell::Empty {
                continue;
            }
            visited[idx] = true;
            reachable_empty += 1;
            queue.push_back((nx, ny));
        }
    }

    reachable_empty
}

fn neighbors(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    DELTAS.into_iter().filter_map(move |(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || ny < 0 || nx >= width as i32 || ny >= height as i32 {
            None
        } else {
            Some((nx as usize, ny as usize))
        }
    })
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

fn validate_foe_placement(
    anfield: &Anfield,
    piece: &Piece,
    x: i32,
    y: i32,
) -> Result<(), PlacementError> {
    if !piece_fits_in_bounds(anfield, piece, x, y) {
        return Err(PlacementError::OutOfBounds);
    }

    let foe_overlaps = count_foe_overlaps(anfield, piece, x, y);
    if has_own_overlap(anfield, piece, x, y) {
        return Err(PlacementError::OpponentOverlap);
    }

    if foe_overlaps != 1 {
        return Err(PlacementError::WrongOverlapCount { got: foe_overlaps });
    }

    Ok(())
}

fn count_foe_overlaps(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> u32 {
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
            if matches!(cell, Cell::Foe | Cell::FoeLast) {
                count += 1;
            }
        }
    }
    count
}

fn has_own_overlap(anfield: &Anfield, piece: &Piece, x: i32, y: i32) -> bool {
    for py in 0..piece.height {
        for px in 0..piece.width {
            if !piece.is_filled(px, py) {
                continue;
            }
            let cell = anfield_cell(anfield, x + px as i32, y + py as i32);
            if matches!(cell, Cell::Own | Cell::OwnLast) {
                return true;
            }
        }
    }
    false
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
    fn count_new_territory_counts_empty_cells_covered_by_piece() {
        let board = anfield(&["...", ".@.", "..."]);
        let line = piece(&["###"]);

        assert_eq!(count_new_territory(&board, &line, 0, 1), 2);
        assert_eq!(count_new_territory(&board, &line, 1, 0), 0);
    }

    #[test]
    fn iter_valid_placements_finds_only_valid_anchors() {
        let board = anfield(&["...", ".@.", "..."]);
        let tet = piece(&["#", "#"]);

        let valid: Vec<_> = iter_valid_placements(&board, &tet, PlayerId::P1).collect();
        assert_eq!(valid, vec![(1, 0), (1, 1)]);
    }
}
