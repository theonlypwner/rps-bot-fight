use strum::EnumCount;

use crate::bot::{Move, Player};

/// Tracks the opponent's previous moves as a Markov model, predicting what their
/// most likely next move is based on what patterns they've played most
/// frequently in the past and what moves they've most recently played.
pub struct MarkovBot {
    root: TrieNode,
}

const ORDER: usize = 10;

impl Player for MarkovBot {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn make_move(&mut self, opp_prev_moves: &[Move]) -> Move {
        if opp_prev_moves.len() <= ORDER {
            return Move::random();
        }

        // Update markov chain values
        let mut current = &mut self.root;
        for &m in opp_prev_moves[opp_prev_moves.len() - 1 - ORDER..].iter() {
            current = current.children[m as usize].get_or_insert_with(|| Box::new(TrieNode::new()))
        }
        current.value += 1;

        // Navigate the trie using the recent moves
        current = &mut self.root;
        for &m in opp_prev_moves[opp_prev_moves.len() - ORDER..].iter() {
            match current.children[m as usize].as_mut() {
                None => return Move::random(),
                Some(c) => current = c,
            }
        }

        // Find the most likely next move that will be played
        let mut next_move = 0;
        let mut value = 0;
        for (i, c) in current.children.iter().enumerate() {
            if let Some(node) = c
                && node.value > value
            {
                value = node.value;
                next_move = i;
            }
        }

        // Play the counter to that move
        return Move::from_repr(next_move as u8).unwrap().get_counter();
    }
}

struct TrieNode {
    value: u64,
    children: [Option<Box<TrieNode>>; Move::COUNT],
}

/// Represents a single move among a series of moves
impl TrieNode {
    pub fn new() -> Self {
        Self {
            value: 0,
            children: [const { None }; Move::COUNT],
        }
    }
}
