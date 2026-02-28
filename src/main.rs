use clap::Parser;

use crate::{bot::*, tournament::TournamentManager};

mod bot;
mod tournament;

#[derive(Parser)]
struct Cli {
    #[arg(value_name = "NUM_ROUNDS", default_value_t = 1000)]
    rounds: u32,

    #[arg(value_name = "NUM_GAMES", default_value_t = 10)]
    games: u32,
}

fn main() {
    let args = Cli::parse();

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

    manager.run_tournament(args.rounds, args.games);
}
