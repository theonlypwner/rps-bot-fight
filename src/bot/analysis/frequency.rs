use strum::EnumCount;

use crate::bot::{Move, Player};

/// Plays the counter to the opponent's most frequent move.
pub struct FrequencyBot {
    counts: [u32; Move::COUNT],
}

impl Player for FrequencyBot {
    fn new() -> Self {
        Self {
            counts: [0; Move::COUNT],
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        match opp_prev_moves.last() {
            None => Move::random(),
            Some(m) => {
                // Record opponent's previous move
                self.counts[*m as usize] += 1;

                // Find the opponent's move frequent move
                let m = self
                    .counts
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, value)| value)
                    .unwrap()
                    .0;

                // Play the counter to that move
                Move::from_repr(m as u8).unwrap().get_counter()
            }
        }
    }
}
