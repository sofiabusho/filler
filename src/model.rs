//! Grid and piece types for the Filler protocol.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerId {
    P1,
    P2,
}

impl PlayerId {
    pub fn from_exec_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::P1),
            2 => Some(Self::P2),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    OwnLast,
    Own,
    FoeLast,
    Foe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anfield {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub mask: Vec<bool>,
}

impl Piece {
    pub fn filled_count(&self) -> usize {
        self.mask.iter().filter(|&&filled| filled).count()
    }
}

/// Engine stdout format: `"{x} {y}\n"`.
pub fn format_move(x: i32, y: i32) -> String {
    format!("{x} {y}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_id_from_exec_number() {
        assert_eq!(PlayerId::from_exec_number(1), Some(PlayerId::P1));
        assert_eq!(PlayerId::from_exec_number(2), Some(PlayerId::P2));
        assert_eq!(PlayerId::from_exec_number(3), None);
    }

    #[test]
    fn format_move_matches_engine_protocol() {
        assert_eq!(format_move(7, 2), "7 2\n");
        assert_eq!(format_move(0, 0), "0 0\n");
    }
}
