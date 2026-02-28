use crate::bot::{Move, Player};

/// Always plays paper.
pub struct PaperDummy {}

impl Player for PaperDummy {
    fn new() -> Self {
        Self {}
    }

    fn make_move(&mut self, _opp_prev_moves: &[Move]) -> Move {
        Move::Paper
    }
}
