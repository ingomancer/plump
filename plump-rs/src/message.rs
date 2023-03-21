use std::collections::HashMap;

use crate::game::{Card, Player, PublicState, Trick};

pub enum Message<'a> {
    RequestGuessContext {
        player: &'a Player<'a>,
        hand: &'a [Card],
        guesses: &'a [u32],
        players: usize,
    },

    Guesses {
        state: &'a HashMap<&'a str, PublicState>,
    },

    Turn {
        whose: &'a Player<'a>,
    },

    PlayRequestContext {
        player: &'a Player<'a>,
        hand: &'a [Card],
        trick: &'a Trick,
    },

    Trick(&'a Trick),

    Scoreboard {
        state: &'a HashMap<&'a str, PublicState>,
    },

    Winner(&'a Player<'a>),

    Winners {
        players: &'a [Player<'a>],
        winner_indices: &'a [usize],
    },
}
