use std::collections::HashMap;

use playing_cards::structs::Card;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Debug)]
pub struct PlayerName(pub String);

impl PlayerName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Player {
    pub name: PlayerName,
    pub human: bool,
    pub hand: Vec<Card>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Trick(pub Vec<Card>);

impl Trick {
    pub const fn new() -> Self {
        Self(Vec::new())
    }
}

impl Default for Trick {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct PublicState {
    pub guess: Option<usize>,
    pub wins: usize,
    pub score: usize,
}

pub type StatePerPlayer = HashMap<PlayerName, PublicState>;
