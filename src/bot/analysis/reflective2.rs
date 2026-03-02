use crate::bot::{
    Move, Player,
    analysis::{reflective::Predictor, sam::SuffixAutomaton},
};

/// Analyzes the opponents move history to find the longest move sequence that
/// matches the most recent move sequence. Then looks at what they played next
/// and plays the counter to it.
///
/// Does the same to own history to predict what an opponent would play if they
/// are doing the same strategy. Keeps track of which version would have
/// performed against what the opponent chose, and uses the best.
pub struct ReflectiveBot2 {
    sam_opp: SuffixAutomaton,
    sam_me: SuffixAutomaton,
    predictor_opp: Predictor,
    predictor_me: Predictor,
}

impl Player for ReflectiveBot2 {
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

                self.predictor_opp.predicted_move = self.sam_opp.predict().get_counter();
                self.predictor_me.predicted_move = self.sam_me.predict().get_defeated();

                if self.predictor_opp.score > self.predictor_me.score {
                    self.predictor_opp.predicted_move
                } else {
                    self.predictor_me.predicted_move
                }
            }
        };
        self.sam_me.push(my_move);
        my_move
    }
}
