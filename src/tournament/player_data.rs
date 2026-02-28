use linked_hash_map::LinkedHashMap;
use std::{
    cmp::Reverse,
    collections::HashMap,
    ops::{Add, AddAssign},
};

use crate::bot::Player;

/// Holds the data for a single player in the tournament.
pub struct PlayerData {
    pub name: String,
    constructor: fn() -> Box<dyn Player>,
    pub rounds_record: Record,
    pub games_record: Record,
    nemeses: HashMap<String, Record>,
    nemesis: Option<(String, usize)>,
}

impl PlayerData {
    pub fn new(name: &str, constructor: fn() -> Box<dyn Player>) -> Self {
        Self {
            name: String::from(name),
            constructor,
            rounds_record: Record::new(),
            games_record: Record::new(),
            nemeses: HashMap::new(),
            nemesis: None,
        }
    }

    /// Updates the nemesis tracker of this player. A player's nemesis is the
    /// player who they've lost the most rounds to.
    pub fn update_nemesis(
        &mut self,
        player_name: String,
        priority: usize,
        losses: u32,
        total: u32,
    ) {
        // Add wins for the nemesis
        let opponent_wins = {
            let opponent_record = &mut self
                .nemeses
                .entry(player_name.clone())
                .or_insert(Record::new());
            opponent_record.wins += losses;
            opponent_record.total += total;
            opponent_record.wins as f64 / opponent_record.total as f64
        };
        match &self.nemesis {
            None => {
                self.nemesis = Some((player_name, priority));
            }
            Some((n, p)) => {
                let nemesis_record = &mut self.nemeses.get_mut(n).unwrap();
                let nemesis_wins = nemesis_record.wins as f64 / nemesis_record.total as f64;
                if (opponent_wins, Reverse(priority)) > (nemesis_wins, Reverse(*p)) {
                    self.nemesis = Some((player_name, priority));
                }
            }
        }
    }

    /// Returns the nemesis of this player. A player's nemesis is the player who
    /// they've lost the most rounds to.
    pub fn get_nemesis(&self) -> String {
        match &self.nemesis {
            Some((n, _)) => n.clone(),
            None => "N/A".to_string(),
        }
    }

    ///  Resets all of this player's win/loss/draw totals for rounds and games, as
    /// well as the player's nemesis tracker.
    pub fn reset_records(&mut self) {
        self.rounds_record = Record::new();
        self.games_record = Record::new();
        self.nemeses = HashMap::new();
        self.nemesis = None;
    }

    /// Returns a collection of stats about this player.
    pub fn get_stats(&self) -> LinkedHashMap<String, String> {
        LinkedHashMap::from_iter([
            ("Name".to_string(), self.name.clone()),
            ("Games Won".to_string(), self.games_record.to_string()),
            ("Rounds Won".to_string(), self.rounds_record.to_string()),
            ("Nemesis".to_string(), self.get_nemesis()),
            (
                "Rounds Lost to Nemesis".to_string(),
                match &self.nemesis {
                    Some((n, _)) => self.nemeses[n].percent(),
                    None => "N/A".to_string(),
                },
            ),
        ])
    }

    /// Creates a new instance of the player whose data is being stored.
    pub fn new_instance(&self) -> Box<dyn Player> {
        (self.constructor)()
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
