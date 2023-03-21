use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{
    game::{playable_card_indices, Card, Player, PublicState, Trick},
    message::Message,
};

const SUIT_SYMBOLS: [&str; 4] = ["♥", "♣", "♦", "♠"];
const CARD_SYMBOLS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

fn darken(text: String) -> String {
    format!("\x1b[90m{text}\x1b[0m")
}

fn format_card(card: &Card, darkened: bool, index: Option<usize>) -> String {
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

fn format_trick(Trick(cards): &Trick) -> Option<String> {
    const DARKENED: bool = false;
    const INDEX: Option<usize> = None;

    (!cards.is_empty()).then(|| {
        cards
            .iter()
            .map(|c| format_card(c, DARKENED, INDEX))
            .join(" ")
    })
}

fn format_hand(hand: &[Card], valid_cards: &Option<HashSet<usize>>, with_indices: bool) -> String {
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

fn format_guesses(state: &HashMap<&str, PublicState>) -> String {
    let guesses = state
        .iter()
        .map(|(name, state)| format!("{}: {}", name, format_guess(state)))
        .join(", ");

    "Guesses: ".to_owned() + &guesses
}

const UPSIDE_DOWN_FACE: char = '\u{1F643}';
const SLIGHTLY_SMILING_FACE: char = '\u{1F642}';

fn format_scoreboard(public: &HashMap<&str, PublicState>) -> String {
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

fn format_player_prompt(trick: &Trick) -> String {
    let trick_string = format_trick(trick);
    match trick_string {
        Some(text) => "Trick: ".to_owned() + &text,
        None => "You go first!".to_owned(),
    }
}

fn format_turn(player: &Player) -> String {
    format!("{}'s turn", &player.name)
}

fn format_winner(player: &Player) -> String {
    format!("{} won!", &player.name)
}

fn format_request_guess_context(
    player: &Player,
    hand: &[Card],
    guesses: &[u32],
    players: usize,
) -> String {
    const VALID_CARDS: Option<HashSet<usize>> = None;
    const WITH_INDICES: bool = false;

    let hand_string = format_hand(hand, &VALID_CARDS, WITH_INDICES);
    let guesses_string = guesses.iter().map(|i| i.to_string()).join(" ");

    format!(
        "{}: Hand: {hand_string}, Previous Guesses: {guesses_string}, Players: {players}",
        &player.name
    )
}

fn format_play_request_context(player: &Player, hand: &[Card], trick: &Trick) -> String {
    const WITH_INDICES: bool = true;
    let valid_cards = playable_card_indices(hand, trick);
    let hand_string = format_hand(hand, &valid_cards, WITH_INDICES);

    let state = format_player_prompt(trick);
    format!("{}: Hand: {hand_string}, {state}", player.name)
}

fn format_winners(players: &[Player], winners: &[usize]) -> String {
    let winners_text = winners.iter().map(|i| players[*i].name).join(", ");
    format!("The winner(s) is/are {winners_text}!")
}

impl<'a> ToString for Message<'a> {
    fn to_string(&self) -> String {
        match self {
            Message::RequestGuessContext {
                player,
                hand,
                guesses,
                players,
            } => format_request_guess_context(player, hand, guesses, *players),

            Message::Guesses { state } => format_guesses(state),

            Message::Turn { whose: player } => format_turn(player),

            Message::PlayRequestContext {
                player,
                hand,
                trick,
            } => format_play_request_context(player, hand, trick),

            Message::Trick(trick) => format_trick(trick).unwrap_or_default(),

            Message::Scoreboard { state } => format_scoreboard(state),

            Message::Winner(player) => format_winner(player),

            Message::Winners {
                players,
                winner_indices,
            } => format_winners(players, winner_indices),
        }
    }
}
