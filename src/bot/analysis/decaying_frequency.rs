use strum::EnumCount;

use crate::bot::{Move, Player};

/// Plays the counter to the opponent's most frequent move. Move frequencies
/// decay over time so more recent moves are weighted higher.
pub struct DecayingFrequencyBot {
    scores: [f64; Move::COUNT],
}

const DECAY: f64 = 0.9;
const CHANGE: f64 = 0.1;

impl Player for DecayingFrequencyBot {
    fn new() -> Self {
        Self {
            scores: [0.0; Move::COUNT],
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        match opp_prev_moves.last() {
            None => Move::random(),
            Some(opponents_last) => {
                // Apply decay to all scores
                for i in self.scores.iter_mut() {
                    *i *= DECAY;
                }

                // Update scores based on opponent's move
                self.scores[opponents_last.get_counter() as usize] += CHANGE;
                self.scores[opponents_last.get_defeated() as usize] -= CHANGE;

                // Find the opponent's move frequent move
                let index_of_most = self
                    .scores
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .unwrap()
                    .0;

                // Play the counter to that move
                Move::from_repr(index_of_most as u8).unwrap()
            }
        }
    }
}
