use rand::RngExt;
use strum_macros::{EnumCount, FromRepr};

pub use {analysis::*, dummy::*};

pub mod analysis;
pub mod dummy;

/// Implement this trait to make a player that can compete in the tournament.
pub trait Player {
    fn new() -> Self
    where
        Self: Sized;

    /// Returns the next move this player will make based on its opponent's previous moves.
    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move;
}

#[derive(Clone, Copy, EnumCount, FromRepr, PartialEq)]
#[repr(u8)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Move {
    pub fn random() -> Move {
        Move::from_repr(rand::rng().random_range(0..3)).unwrap()
    }

    pub fn beats(self, other: Move) -> bool {
        other.get_counter() == self
    }

    pub fn versus(self, other: Move) -> Outcome {
        if self == other {
            Outcome::Draw
        } else if self.beats(other) {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    pub fn get_counter(self) -> Self {
        self.shift(1)
    }

    pub fn get_defeated(self) -> Self {
        self.shift(2)
    }

    pub fn shift(self, amount: u8) -> Self {
        Move::from_repr((self as u8 + amount) % 3).unwrap()
    }
}
