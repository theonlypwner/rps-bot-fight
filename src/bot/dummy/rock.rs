use crate::bot::{Move, Player};

/// Always plays rock.
pub struct RockDummy {}

impl Player for RockDummy {
    fn new() -> Self {
        Self {}
    }

    fn make_move(&mut self, _opp_prev_moves: &[Move]) -> Move {
        Move::Rock
    }
}
