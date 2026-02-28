use std::{
    any::type_name,
    cmp::Reverse,
    io::{self, Write},
};

use crate::{
    bot::{Move, Outcome, Player},
    tournament::player_data::PlayerData,
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

        let total_games =
            (num_games as usize) * (self.players.len() * (self.players.len() - 1) / 2);

        let game_per_percent: f64 = 1.0 / (total_games as f64);
        let mut percent_done: f64 = 0.0;
        let mut percent_done_shown: f64 = 0.0;
        print!("Tournament Progress: 0%");

        for i in 0..self.players.len() {
            for j in i + 1..self.players.len() {
                for _game in 0..num_games {
                    self.play_game(i, j, num_rounds);

                    // Update tournament completion percentage
                    percent_done += game_per_percent;
                    while percent_done_shown < percent_done {
                        percent_done_shown += COMPLETION_PERCENT_UPDATE;
                        if percent_done_shown <= 1.0 {
                            print!(" {:.0}%", percent_done_shown * 100.0);
                            io::stdout().flush().expect("Unable to flush stdout");
                        }
                    }
                }
            }
        }
        print!("\n\n");

        self.print_rankings();
    }

    /// Plays a single game consisting of the given number of rounds between the
    /// two given players. Player data for both players is updated depending on
    /// the results of the rounds and the game.
    fn play_game(&mut self, i: usize, j: usize, num_rounds: u32) {
        let (part1, part2) = self.players.split_at_mut(j);

        let player1_data = &mut part1[i];
        let player2_data = &mut part2[0];

        let mut player1 = player1_data.new_instance();
        let mut player2 = player2_data.new_instance();

        let mut player1_moves: Vec<Move> = vec![];
        let mut player2_moves: Vec<Move> = vec![];
        let mut player1_round_wins = 0;
        let mut player2_round_wins = 0;

        // Play the given number of rounds
        for _round in 0..num_rounds {
            let player1_move = player1.make_move(player2_moves.as_slice());
            let player2_move = player2.make_move(player1_moves.as_slice());

            player1_moves.push(player1_move);
            player2_moves.push(player2_move);

            let result = player1_move.versus(player2_move);
            match result {
                Outcome::Draw => {
                    // Draw
                    player1_data.rounds_record.add_draw();
                    player2_data.rounds_record.add_draw();
                }
                Outcome::Win => {
                    // Player 1 wins
                    player1_round_wins += 1;
                    player1_data.rounds_record.add_win();
                    player2_data.rounds_record.add_loss();
                }
                Outcome::Loss => {
                    // Player 2 wins
                    player2_round_wins += 1;
                    player1_data.rounds_record.add_loss();
                    player2_data.rounds_record.add_win();
                }
            }
        }

        // Update game records based on game results
        if player1_round_wins > player2_round_wins {
            player1_data.games_record.add_win();
            player2_data.games_record.add_loss();
        } else if player1_round_wins < player2_round_wins {
            player1_data.games_record.add_loss();
            player2_data.games_record.add_win();
        } else {
            player1_data.games_record.add_draw();
            player2_data.games_record.add_draw();
        }
        player1_data.update_nemesis(player2_data.name.clone(), player2_round_wins, num_rounds);
        player2_data.update_nemesis(player1_data.name.clone(), player1_round_wins, num_rounds);
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
        let column_names: Vec<String> = Vec::from_iter(stats[0].keys().map(|s| s.clone()));
        let column_name_length: Vec<usize> =
            Vec::from_iter(column_names.iter().map(|column_name| {
                std::cmp::max(
                    column_name.len(),
                    stats
                        .iter()
                        .map(|player_stats| player_stats[column_name].len())
                        .max()
                        .unwrap(),
                ) + COLUMN_SPACING
            }));

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
            for (i, column_name) in column_names.iter().enumerate() {
                let w = column_name_length[i];
                print!("{:w$}", player_stats[column_name]);
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
