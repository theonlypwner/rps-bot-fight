use strum::EnumCount;

use crate::bot::{Move, Player, analysis::sam::SuffixAutomaton};

/// Analyzes the opponents move history to find the longest move sequence that
/// matches the most recent move sequence. Then looks at what they played next
/// and plays the counter to it.
///
/// Keeps track of how different "shifts" to its chosen move would have performed
/// against what the opponent chose, and applies the best.
pub struct MetaBot {
    sam: SuffixAutomaton<u8>,
    base_move: Move,
    shifts: [f64; Move::COUNT],
}

const DECAY: f64 = 0.9;
const CHANGE: f64 = 0.1;

impl Player for MetaBot {
    fn new() -> Self {
        Self {
            sam: SuffixAutomaton::new(),
            base_move: Move::random(),
            shifts: [0.0; Move::COUNT],
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        match opp_prev_moves.last() {
            None => self.base_move,
            Some(&opp_last_move) => {
                update_strategy(self, opp_last_move);

                self.sam.push(opp_last_move as u8);
                self.base_move = Move::from_repr(self.sam.predict()).unwrap().get_counter();

                self.base_move.shift(
                    self.shifts
                        .iter()
                        .enumerate()
                        .max_by(|(_, a), (_, b)| a.total_cmp(b))
                        .unwrap()
                        .0 as u8,
                )
            }
        }
    }
}

fn update_strategy(bot: &mut MetaBot, opp_last_move: Move) {
    for shift in 0..bot.shifts.len() {
        bot.shifts[shift] *= DECAY;
        let shifted_move = bot.base_move.shift(shift as u8);
        if shifted_move.beats(opp_last_move) {
            // Win
            bot.shifts[shift] += CHANGE;
        } else if shifted_move != opp_last_move {
            // Lose
            bot.shifts[shift] -= CHANGE;
        }
    }
}
