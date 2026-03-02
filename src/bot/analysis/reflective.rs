use crate::bot::{Move, Player, analysis::sam::SuffixAutomaton};

/// Analyzes the opponents move history to find the longest move sequence that
/// matches the most recent move sequence. Then looks at what they played next
/// and plays the counter to it.
///
/// Does the same to own history to predict what an opponent would play if they
/// are doing the same strategy. Keeps track of which version would have
/// performed against what the opponent chose, and uses the best.
pub struct ReflectiveBot {
    sam_opp: SuffixAutomaton,
    sam_me: SuffixAutomaton,
    predictor_opp: Predictor,
    predictor_me: Predictor,
}

const DECAY: f64 = 0.9;
const CHANGE: f64 = 0.1;

impl Player for ReflectiveBot {
    fn new() -> Self {
        Self {
            sam_opp: SuffixAutomaton::new(),
            sam_me: SuffixAutomaton::new(),
            predictor_opp: Predictor::new(),
            predictor_me: Predictor::new(),
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        let my_move = match opp_prev_moves.last() {
            None => {
                let next_move = Move::random();

                self.predictor_opp.predicted_move = next_move;
                self.predictor_me.predicted_move = next_move;

                next_move
            }
            Some(&opp_last_move) => {
                self.sam_opp.push(opp_last_move);

                self.predictor_opp.update(opp_last_move);
                self.predictor_me.update(opp_last_move);

                self.predictor_opp.predicted_move = self.sam_opp.predict();
                self.predictor_me.predicted_move = self.sam_me.predict();

                if self.predictor_opp.score > self.predictor_me.score {
                    self.predictor_opp.predicted_move.get_counter()
                } else {
                    self.predictor_me.predicted_move.get_defeated()
                }
            }
        };
        self.sam_me.push(my_move);
        my_move
    }
}

struct Predictor {
    predicted_move: Move,
    score: f64,
}

impl Predictor {
    fn new() -> Self {
        Self {
            predicted_move: Move::Rock,
            score: 0.0,
        }
    }

    fn update(&mut self, last_move: Move) {
        self.score *= DECAY;
        if self.predicted_move.beats(last_move) {
            // Won
            self.score += CHANGE
        } else if self.predicted_move != last_move {
            // Lost
            // self.score -= CHANGE
            self.score = 0.0
        }
    }
}
