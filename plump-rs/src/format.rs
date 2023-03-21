use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::game::{Card, PublicState, Trick};

const SUIT_SYMBOLS: [&str; 4] = ["♥", "♣", "♦", "♠"];
const CARD_SYMBOLS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

fn darken(text: String) -> String {
    format!("\x1b[90m{text}\x1b[0m")
}

pub fn format_card(card: &Card, darkened: bool, index: Option<usize>) -> String {
    let suit_symbol = SUIT_SYMBOLS[card.suit as usize];
    let card_symbol = CARD_SYMBOLS[card.value as usize];

    let index_string = match index {
        None => "".into(),
        Some(index) => format!("{index}|"),
    };

    let text = format!("{index_string}{suit_symbol}{card_symbol}");

    match darkened {
        true => darken(text),
        false => text,
    }
}

pub fn format_trick(Trick(cards): &Trick) -> Option<String> {
    const DARKENED: bool = false;
    const INDEX: Option<usize> = None;

    (!cards.is_empty()).then(|| {
        cards
            .iter()
            .map(|c| format_card(c, DARKENED, INDEX))
            .join(" ")
    })
}

pub fn format_hand(
    hand: &[Card],
    valid_cards: &Option<HashSet<usize>>,
    with_indices: bool,
) -> String {
    hand.iter()
        .enumerate()
        .map(|(index, card)| {
            let darkened = match &valid_cards {
                Some(cards) => !cards.contains(&index),
                None => false,
            };

            let index = with_indices.then_some(index);
            format_card(card, darkened, index)
        })
        .join(" ")
}

fn format_guess(state: &PublicState) -> String {
    state.guess.map(|g| g.to_string()).unwrap_or("?".into())
}

pub fn format_guesses(state: &HashMap<&str, PublicState>) -> String {
    let guesses = state
        .iter()
        .map(|(name, state)| format!("{}: {}", name, format_guess(state)))
        .join(", ");

    "Guesses: ".to_owned() + &guesses
}

const UPSIDE_DOWN_FACE: char = '\u{1F643}';
const SLIGHTLY_SMILING_FACE: char = '\u{1F642}';

pub fn format_scoreboard(public: &HashMap<&str, PublicState>) -> String {
    fn format_state(public: &PublicState) -> String {
        let PublicState { guess, wins, score } = *public;
        let did_plump = guess.filter(|guess| wins == *guess).is_none();

        let face = match did_plump {
            false => SLIGHTLY_SMILING_FACE,
            true => UPSIDE_DOWN_FACE,
        };

        let guess_text = format_guess(public);
        format!("{wins}/{guess_text} {face} (total: {score})")
    }

    public
        .keys()
        .sorted()
        .map(|name| {
            let state = format_state(public.get(name).unwrap());
            format!("{name}: {state}")
        })
        .join(", ")
}
