use crate::bot::{Move, Player};

/// Always plays a random move.
pub struct RandomDummy {}

impl Player for RandomDummy {
    fn new() -> Self {
        Self {}
    }

    fn make_move(&mut self, _opp_prev_moves: &[Move]) -> Move {
        Move::random()
    }
}
