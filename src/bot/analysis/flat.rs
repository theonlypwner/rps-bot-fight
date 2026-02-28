use rand::RngExt;
use strum::EnumCount;

use crate::bot::{Move, Player};

/// Chooses moves randomly, but with a bias towards moves chosen less frequently,
/// ultimately attempting to have a perfectly flat move frequency distribution.
pub struct FlatBot {
    counts: [u32; Move::COUNT],
    total: u32,
}

impl Player for FlatBot {
    fn new() -> Self {
        Self {
            counts: [0; Move::COUNT],
            total: 0,
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        self.total += 1;

        let m = match opp_prev_moves.last() {
            None => Move::random(),
            Some(_) => {
                let weights = self.counts.iter().map(|c| self.total * 2 - c);

                let r = rand::rng().random_range(0..self.total * 2);
                let mut sum = 0;
                Move::from_repr(
                    weights
                        .enumerate()
                        .find(|&(_, item)| {
                            sum += item;
                            sum >= r
                        })
                        .unwrap()
                        .0 as u8,
                )
                .unwrap()
            }
        };

        // Update count
        self.counts[m as usize] += 1;
        m
    }
}
