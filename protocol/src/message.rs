use std::collections::HashSet;

use playing_cards::structs::Card;
use serde::{Deserialize, Serialize};

use crate::structs::{Player, StatePerPlayer, Trick};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    RequestGuessContext {
        player: Player,
        hand: Vec<Card>,
        guesses: Vec<usize>,
        players: usize,
    },

    Guesses {
        state: StatePerPlayer,
    },

    Turn {
        whose: Player,
    },

    PlayRequestContext {
        player: Player,
        hand: Vec<Card>,
        trick: Trick,
        valid_cards: Option<HashSet<usize>>,
    },

    Trick(Trick),

    Scoreboard {
        state: StatePerPlayer,
    },

    Winner(Player),

    Winners {
        players: Vec<Player>,
        winner_indices: Vec<usize>,
    },
    RequestPlayerName,
    PlayRequest(Player),
    RequestGuess,
    GameOver,
}
