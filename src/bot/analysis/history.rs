use crate::bot::{Move, Player, analysis::sam::SuffixAutomaton};

/// Analyzes the opponents move history to find the longest move sequence that
/// matches the most recent move sequence. Then looks at what they played next
/// and plays the counter to it.
pub struct HistoryBot {
    sam: SuffixAutomaton<u8>,
}

impl Player for HistoryBot {
    fn new() -> Self {
        Self {
            sam: SuffixAutomaton::new(),
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        match opp_prev_moves.last() {
            None => Move::random(),
            // Some(_) => _move_slow(opp_prev_moves),
            Some(&opp_last_move) => Move::from_repr(self.sam.push(opp_last_move as u8))
                .unwrap()
                .get_counter(),
        }
    }
}

fn _index_of_next_slow(opp_prev_moves: &[Move]) -> usize {
    const _MAX_SEARCH_DEPTH: usize = 1000;

    // Most recent move
    let history_end = opp_prev_moves.len() - 1;
    // How far back to search
    let search_depth = std::cmp::min(opp_prev_moves.len(), _MAX_SEARCH_DEPTH);
    // Length of the longest match sequence
    let mut longest_match = 0;
    // Index of the next move played after the longest match sequence
    let mut index_of_next = 0;

    // Search history for matches of increasing sequence length
    for i in 1..search_depth {
        let mut high = history_end;
        let mut low = (high - i) as usize;

        while opp_prev_moves[low] == opp_prev_moves[high] {
            high -= 1;
            if low == 0 {
                // original code does not have this fix
                break;
            }
            low -= 1;
        }
        if history_end - high > longest_match {
            longest_match = history_end - high;
            index_of_next = history_end - i + 1;
        }
    }
    index_of_next
}

fn _move_slow(opp_prev_moves: &[Move]) -> Move {
    opp_prev_moves[_index_of_next_slow(opp_prev_moves)].get_counter()
}
