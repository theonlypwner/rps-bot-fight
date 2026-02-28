use rand::RngExt;

use crate::bot::{Move, Player};

/// Creates a random pattern of moves before the game begins, then plays it
/// repeatably for the entirety of the game.
pub struct PatternDummy<const MIN_SIZE: usize = 5, const MAX_SIZE: usize = 15> {
    pattern: Vec<Move>,
    index: usize,
}

impl<const MIN_SIZE: usize, const MAX_SIZE: usize> Player for PatternDummy<MIN_SIZE, MAX_SIZE> {
    fn new() -> Self {
        let n = rand::rng().random_range(MIN_SIZE..=MAX_SIZE);
        let mut pattern: Vec<Move> = Vec::with_capacity(n);

        for _ in 0..n {
            pattern.push(Move::random())
        }

        Self {
            pattern: pattern,
            index: 0,
        }
    }

    fn make_move(&mut self, _opp_prev_moves: &[Move]) -> Move {
        self.index += 1;
        if self.index == self.pattern.len() {
            self.index = 0
        }

        self.pattern[self.index]
    }
}
