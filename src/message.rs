use crate::game::{Card, Player, StatePerPlayer, Trick};

pub enum Message<'a> {
    RequestGuessContext {
        player: &'a Player<'a>,
        hand: &'a [Card],
        guesses: &'a [u32],
        players: usize,
    },

    Guesses {
        state: &'a StatePerPlayer<'a>,
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
        state: &'a StatePerPlayer<'a>,
    },

    Winner(&'a Player<'a>),

    Winners {
        players: &'a [Player<'a>],
        winner_indices: &'a [usize],
    },
}
