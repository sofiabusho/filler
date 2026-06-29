//! Integration smoke tests for the quality gate (T02).

use filler::model::format_move;

#[test]
fn format_move_integration() {
    assert_eq!(format_move(12, 34), "12 34\n");
}
