use std::{
    cmp::Reverse,
    ops::{Add, AddAssign},
};

use crate::bot::Player;

/// Holds the data for a single player in the tournament.
pub struct PlayerData {
    pub name: String,
    constructor: fn() -> Box<dyn Player>,
    pub rounds_record: Record,
    pub games_record: Record,
    opponents: Vec<OpponentData>,
}

struct OpponentData {
    name: String,
    priority: usize,
    rounds_record: Record,
}

pub struct PlayerStats {
    name: String,
    games_won: String,
    rounds_won: String,
    nemesis: String,
    nemesis_rounds: String,
}

impl PlayerData {
    pub fn new(name: &str, constructor: fn() -> Box<dyn Player>) -> Self {
        Self {
            name: String::from(name),
            constructor,
            rounds_record: Record::new(),
            games_record: Record::new(),
            opponents: Vec::new(),
        }
    }

    pub fn add_opponent(&mut self, player_name: String, priority: usize, rounds_record: Record) {
        self.opponents.push(OpponentData {
            name: player_name.clone(),
            priority,
            rounds_record,
        })
    }

    /// Returns the nemesis of this player. A player's nemesis is the player who
    /// they've lost the most rounds to.
    fn get_nemesis(&self) -> Option<&OpponentData> {
        self.opponents
            .iter()
            .max_by_key(|o| (o.rounds_record.losses, Reverse(o.priority)))
    }

    ///  Resets all of this player's win/loss/draw totals for rounds and games, as
    /// well as the player's nemesis tracker.
    pub fn reset_records(&mut self) {
        self.rounds_record = Record::new();
        self.games_record = Record::new();
        self.opponents.clear();
    }

    /// Returns a collection of stats about this player.
    pub fn get_stats(&self) -> PlayerStats {
        let nemesis = self.get_nemesis();

        let nemesis_name = match nemesis {
            Some(n) => n.name.clone(),
            None => "N/A".to_string(),
        };

        let nemesis_losses = match nemesis {
            Some(n) => n.rounds_record.opponent().to_string(),
            None => "N/A".to_string(),
        };

        PlayerStats {
            name: self.name.clone(),
            games_won: self.games_record.to_string(),
            rounds_won: self.rounds_record.to_string(),
            nemesis: nemesis_name,
            nemesis_rounds: nemesis_losses,
        }
    }

    /// Creates a new instance of the player whose data is being stored.
    pub fn new_instance(&self) -> Box<dyn Player> {
        (self.constructor)()
    }
}

impl PlayerStats {
    const COLUMNS: [(&str, fn(&PlayerStats) -> &String); 5] = [
        ("Name", |p| &p.name),
        ("Games Won", |p| &p.games_won),
        ("Rounds Won", |p| &p.rounds_won),
        ("Nemesis", |p| &p.nemesis),
        ("Rounds Lost to Nemesis", |p| &p.nemesis_rounds),
    ];

    pub fn column_names() -> [&'static str; Self::COLUMNS.len()] {
        Self::COLUMNS.map(|c| c.0)
    }

    pub fn column_name_length(stats: &[PlayerStats], add: usize) -> [usize; Self::COLUMNS.len()] {
        Self::COLUMNS.map(|(column_name, getter)| {
            add + std::iter::once(column_name.len())
                .chain(stats.iter().map(|player_stats| getter(player_stats).len()))
                .max()
                .unwrap()
        })
    }

    pub fn get_value(col: usize, player_stats: &PlayerStats) -> &String {
        return Self::COLUMNS[col].1(&player_stats);
    }
}

/// Stores totals for win/loss/draw data.
#[derive(Clone, PartialOrd, Eq, PartialEq)]
pub struct Record {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
    pub total: u32,
}

impl Record {
    pub fn new() -> Self {
        Self {
            wins: 0,
            losses: 0,
            draws: 0,
            total: 0,
        }
    }

    pub fn add_win(&mut self) {
        self.wins += 1;
        self.total += 1;
    }

    pub fn add_loss(&mut self) {
        self.losses += 1;
        self.total += 1;
    }

    pub fn add_draw(&mut self) {
        self.draws += 1;
        self.total += 1;
    }

    /// Returns a string containing the fractions of wins and percent of wins
    /// to 1 decimal place.
    pub fn to_string(&self) -> String {
        let length = self.total.to_string().len();
        let wins = self.wins.to_string();

        let total = self.total;
        let p = self.percent();
        format!("{wins:>length$}/{total} ({p})")
    }

    /// Returns the percent of wins to 1 decimal place.
    pub fn percent(&self) -> String {
        let p = 100.0 * self.wins as f64 / self.total as f64;
        format!("{p:.1}%")
    }

    // Returns the record for the opponent
    pub fn opponent(&self) -> Self {
        Self {
            wins: self.losses,
            losses: self.wins,
            draws: self.draws,
            total: self.total,
        }
    }
}

impl Add for Record {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            wins: self.wins + other.wins,
            losses: self.losses + other.losses,
            draws: self.draws + other.draws,
            total: self.total + other.total,
        }
    }
}

impl AddAssign for Record {
    fn add_assign(&mut self, other: Self) {
        self.wins += other.wins;
        self.losses += other.losses;
        self.draws += other.draws;
        self.total += other.total;
    }
}
