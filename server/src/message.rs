use serde::Serialize;

use crate::game::{Card, Player, StatePerPlayer, Trick};

#[derive(Serialize, Copy, Clone)]
pub enum Message<'a> {
    RequestGuessContext {
        player: &'a Player,
        hand: &'a [Card],
        guesses: &'a [usize],
        players: usize,
    },

    Guesses {
        state: &'a StatePerPlayer<'a>,
    },

    Turn {
        whose: &'a Player,
    },

    PlayRequestContext {
        player: &'a Player,
        hand: &'a [Card],
        trick: &'a Trick,
    },

    Trick(&'a Trick),

    Scoreboard {
        state: &'a StatePerPlayer<'a>,
    },

    Winner(&'a Player),

    Winners {
        players: &'a [Player],
        winner_indices: &'a [usize],
    },
    RequestPlayerName,
    PlayRequest(&'a Player),
    RequestGuess,
}
