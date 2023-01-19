use std::collections::HashMap;

use itertools::Itertools;

use crate::game::{Card, PublicState};

static suit_symbols: [&str; 4] = ["♥", "♣", "♦", "♠"];
static card_symbols: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

pub fn format_hand(hand: &Vec<Card>) -> String {
    hand.iter()
        .map(|x| {
            format!(
                "{}{}",
                suit_symbols[x.suit as usize], card_symbols[x.value as usize]
            )
        })
        .join(" ")
}

pub fn format_guesses(state: &HashMap<&str, PublicState>) -> String {
    let guesses = state
        .iter()
        .map(|(name, state)| format!("{}: {}", name, state.guess))
        .join(", ");
    return "Guesses: ".to_owned() + &guesses;
}
