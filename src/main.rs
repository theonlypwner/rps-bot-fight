use crate::{bot::*, tournament::TournamentManager};

mod bot;
mod tournament;

const NUM_ROUNDS: u32 = 1000;
const NUM_GAMES: u32 = 10;

fn main() {
    let mut manager = TournamentManager::new();

    manager.add::<RandomDummy>();
    manager.add::<RockDummy>();
    manager.add::<PaperDummy>();
    manager.add::<ScissorsDummy>();
    manager.add::<PatternDummy<5, 15>>();
    manager.add::<DeBruijnDummy>();

    manager.add::<FrequencyBot>();
    manager.add::<DecayingFrequencyBot>();
    manager.add::<HistoryBot>();
    manager.add::<MarkovBot>();
    manager.add::<ReflectiveBot>();
    manager.add::<MetaBot>();
    manager.add::<BiasBot>();
    manager.add::<FlatBot>();

    manager.run_tournament(NUM_ROUNDS, NUM_GAMES);
}
