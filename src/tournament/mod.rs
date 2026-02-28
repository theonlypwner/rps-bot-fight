use std::{
    any::type_name,
    cmp::Reverse,
    io::{self, Write},
    sync::Mutex,
};

use rayon::prelude::*;

use crate::{
    bot::{Move, Outcome, Player},
    tournament::player_data::{PlayerData, PlayerStats, Record},
};

mod player_data;

/// Manages a tournament between rock-paper-scissors players. Tournaments consist
/// of round-robin play wherein every player plays a match against every other
/// player. A match consists of a set number of games, with each game lasting a
/// set number of rounds.
///
/// Victors are determined as the player with the most games won. Ties are broken
/// by number of individual rounds won.
pub struct TournamentManager {
    players: Vec<PlayerData>,
}

const COLUMN_SPACING: usize = 3;
const COMPLETION_PERCENT_UPDATE: f64 = 0.1;

impl TournamentManager {
    /// Initializes a new tournament with no players.
    pub fn new() -> Self {
        Self { players: vec![] }
    }

    /// Adds a player to the tournament.
    pub fn add<T: Player + 'static>(&mut self) {
        let name = type_name::<T>();
        self.players.push(PlayerData::new(
            name.split("::").last().unwrap_or(name),
            || Box::new(T::new()),
        ));
    }

    /// Runs a round-robin tournament between all loaded player. Each player
    /// plays a match against every other player. A match consists of the given
    /// number of games, with each game lasting the given number of rounds.
    pub fn run_tournament(&mut self, num_rounds: u32, num_games: u32) {
        if self.players.len() < 2 {
            panic!("At least 2 players must be added to be able to run a tournament.")
        }

        self.reset_player_data();

        println!("Playing tournament with:");
        println!("\t{num_rounds} round long games");
        println!("\t{num_games} game long matches");
        println!("\t{} competitors", self.players.len());
        println!();

        let pairs = self.players.len() * (self.players.len() - 1) / 2;
        let total_games = num_games as u64 * pairs as u64;

        let progress = &Mutex::new(TournamentProgress::new(total_games));
        print!("Tournament Progress:");

        (0..self.players.len())
            .into_par_iter()
            .flat_map(|i| {
                (i + 1..self.players.len())
                    .into_par_iter()
                    .map(move |j| (i, j))
            })
            .map(|(i, j)| {
                let (rounds, games) = (0..num_games)
                    .into_par_iter()
                    .map(|_| {
                        let rounds_cur = TournamentManager::play_game(
                            &self.players[i],
                            &self.players[j],
                            num_rounds,
                        );

                        // Update game records based on game results
                        let mut games_cur = Record::new();
                        games_cur.add_outcome(match rounds_cur.wins.cmp(&rounds_cur.losses) {
                            std::cmp::Ordering::Greater => Outcome::Win,
                            std::cmp::Ordering::Less => Outcome::Loss,
                            std::cmp::Ordering::Equal => Outcome::Draw,
                        });

                        // Update tournament completion percentage
                        progress.lock().unwrap().finish_game();

                        (rounds_cur, games_cur)
                    })
                    .reduce(
                        || (Record::new(), Record::new()),
                        |(a_r, a_g), (b_r, b_g)| (a_r + b_r, a_g + b_g),
                    );

                (i, j, rounds, games)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(i, j, rounds, games)| {
                self.update_player_results(i, j, rounds, games);
            });

        print!("\n\n");

        self.print_rankings();
    }

    /// Plays a single game consisting of the given number of rounds between the
    /// two given players. Player data for both players is updated depending on
    /// the results of the rounds and the game.
    ///
    /// Returns the round record for the first player.
    fn play_game(data1: &PlayerData, data2: &PlayerData, num_rounds: u32) -> Record {
        let mut player1 = data1.new_instance();
        let mut player2 = data2.new_instance();

        let mut player1_moves: Vec<Move> = vec![];
        let mut player2_moves: Vec<Move> = vec![];

        // Play the given number of rounds
        let mut player1_rounds_record = Record::new();
        for _round in 0..num_rounds {
            let player1_move = player1.make_move(player2_moves.as_slice());
            let player2_move = player2.make_move(player1_moves.as_slice());

            player1_moves.push(player1_move);
            player2_moves.push(player2_move);

            let result = player1_move.versus(player2_move);
            player1_rounds_record.add_outcome(result);
        }

        player1_rounds_record
    }

    fn update_player_results(
        &mut self,
        i: usize,
        j: usize,
        player1_rounds: Record,
        player1_games: Record,
    ) {
        let (part1, part2) = self.players.split_at_mut(j);

        let player1_data = &mut part1[i];
        let player2_data = &mut part2[0];

        player1_data.rounds_record += player1_rounds.clone();
        player2_data.rounds_record += player1_rounds.opponent();

        player1_data.games_record += player1_games.clone();
        player2_data.games_record += player1_games.opponent();

        player1_data.add_opponent(player2_data.name.clone(), j, player1_rounds.clone());
        player2_data.add_opponent(player1_data.name.clone(), i, player1_rounds.opponent());
    }

    /// Prints player rankings in column form, displaying the number of games and
    /// rounds won for each player as a fraction and percentage. Players are
    /// ordered in descending order determined first by number of games won, then
    /// by number of rounds won, then alphabetically by name. The nemesis of each
    /// player is additionally shown, this being the player each player has lost
    /// the most rounds to.
    fn print_rankings(&mut self) {
        self.players.sort_by_key(|p| {
            (
                Reverse(p.games_record.wins),
                Reverse(p.rounds_record.wins),
                p.name.clone(),
            )
        });

        // Get the stats of all players
        let stats = Vec::from_iter(self.players.iter().map(|p| p.get_stats()));

        // Get the names of all stats and the width of each stat column
        let column_names = PlayerStats::column_names();
        let column_name_length = PlayerStats::column_name_length(stats.as_slice(), COLUMN_SPACING);

        // Get the total width of the stats table
        // Account for fencepost problem by subtracting
        let total_length = column_name_length.iter().sum::<usize>() - COLUMN_SPACING;

        // Print column names
        for (i, column_name) in column_names.iter().enumerate() {
            let w = column_name_length[i];
            print!("{column_name:w$}");
        }
        println!();

        // Print column header separator bar
        println!("{}", "=".repeat(total_length));

        // Print stats for each player
        for player_stats in stats {
            for (i, w) in column_name_length.iter().enumerate() {
                print!("{:w$}", PlayerStats::get_value(i, &player_stats));
            }
            println!();
        }
    }

    /// Resets the win/loss/draw totals for all players.
    fn reset_player_data(&mut self) {
        for p in &mut self.players {
            p.reset_records()
        }
    }
}

impl Record {
    pub fn add_outcome(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Draw => self.add_draw(),
            Outcome::Win => self.add_win(),
            Outcome::Loss => self.add_loss(),
        }
    }
}

struct TournamentProgress {
    done: u64,
    total: u64,
    percent_next: f64,
}

impl TournamentProgress {
    pub fn new(total: u64) -> Self {
        Self {
            done: 0,
            total,
            percent_next: 0.0,
        }
    }

    fn finish_game(&mut self) {
        self.done += 1;

        while self.percent_next <= self.done as f64 / self.total as f64 {
            print!(" {:.0}%", self.percent_next * 100.0);
            io::stdout().flush().expect("Unable to flush stdout");

            self.percent_next += COMPLETION_PERCENT_UPDATE;
        }
    }
}
