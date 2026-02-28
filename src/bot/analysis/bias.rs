use crate::bot::{Move, Player};

/// Plays the counter to a random move previously made by the opponent.
pub struct BiasBot {}

impl Player for BiasBot {
    fn new() -> Self {
        Self {}
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        opp_prev_moves
            .last()
            .map_or(Move::random(), |m| m.get_counter())
    }
}
