use crate::bot::{Move, Player};

/// Always plays scissors.
pub struct ScissorsDummy {}

impl Player for ScissorsDummy {
    fn new() -> Self {
        Self {}
    }

    fn make_move(&mut self, _opp_prev_moves: &[Move]) -> Move {
        Move::Scissors
    }
}
