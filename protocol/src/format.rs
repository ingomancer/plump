use std::collections::HashSet;

use crate::{
    message::Message,
    structs::{Player, PublicState, StatePerPlayer, Trick},
};
use itertools::Itertools;
use playing_cards::structs::Card;

const SUIT_SYMBOLS: [&str; 4] = ["♥", "♣", "♦", "♠"];
const CARD_SYMBOLS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
];

fn darken(text: &str) -> String {
    format!("\x1b[90m{text}\x1b[0m")
}

fn format_card(card: Card, darkened: bool, index: Option<usize>) -> String {
    let suit_symbol = SUIT_SYMBOLS[card.suit];
    let card_symbol = CARD_SYMBOLS[card.value];

    let index_string = index.map_or_else(String::new, |index| format!("{index}|"));

    let text = format!("{index_string}{suit_symbol}{card_symbol}");

    if darkened {
        darken(&text)
    } else {
        text
    }
}

fn format_trick(Trick(cards): &Trick) -> Option<String> {
    const DARKENED: bool = false;
    const INDEX: Option<usize> = None;

    (!cards.is_empty()).then(|| {
        cards
            .iter()
            .map(|c| format_card(*c, DARKENED, INDEX))
            .join(" ")
    })
}

fn format_hand(hand: &[Card], valid_cards: &Option<HashSet<usize>>, with_indices: bool) -> String {
    hand.iter()
        .enumerate()
        .map(|(index, card)| {
            let darkened = valid_cards
                .as_ref()
                .map_or(false, |cards| !cards.contains(&index));

            let index = with_indices.then_some(index);
            format_card(*card, darkened, index)
        })
        .join(" ")
}

fn format_guess(state: &PublicState) -> String {
    state.guess.map_or_else(|| "?".into(), |g| g.to_string())
}

fn format_guesses(state: &StatePerPlayer) -> String {
    let guesses = state
        .iter()
        .map(|(name, state)| format!("{}: {}", name.as_str(), format_guess(state)))
        .join(", ");

    "Guesses: ".to_owned() + &guesses
}

const UPSIDE_DOWN_FACE: char = '\u{1F643}';
const SLIGHTLY_SMILING_FACE: char = '\u{1F642}';

fn format_scoreboard(public: &StatePerPlayer) -> String {
    fn format_state(public: &PublicState) -> String {
        let PublicState { guess, wins, score } = *public;
        let did_plump = guess.filter(|guess| wins == *guess).is_none();

        let face = if did_plump {
            UPSIDE_DOWN_FACE
        } else {
            SLIGHTLY_SMILING_FACE
        };

        let guess_text = format_guess(public);
        format!("{wins}/{guess_text} {face} (total: {score})")
    }

    public
        .keys()
        .sorted()
        .map(|name| {
            let state = format_state(public.get(name).unwrap());
            format!("{}: {state}", name.as_str())
        })
        .join(", ")
}

fn format_player_prompt(trick: &Trick) -> String {
    let trick_string = format_trick(trick);
    trick_string.map_or_else(
        || "You go first!".to_owned(),
        |text| "Trick: ".to_owned() + &text,
    )
}

fn format_turn(player: &Player) -> String {
    format!("{}'s turn", player.name.as_str())
}

fn format_winner(player: &Player) -> String {
    format!("{} won!", player.name.as_str())
}

fn format_request_guess_context(
    player: &Player,
    hand: &[Card],
    guesses: &[usize],
    players: usize,
) -> String {
    const VALID_CARDS: Option<HashSet<usize>> = None;
    const WITH_INDICES: bool = false;

    let hand_string = format_hand(hand, &VALID_CARDS, WITH_INDICES);
    let guesses_string = guesses
        .iter()
        .map(std::string::ToString::to_string)
        .join(" ");

    format!(
        "{}: Hand: {hand_string}, Previous Guesses: {guesses_string}, Players: {players}",
        player.name.as_str()
    )
}

fn format_request_guess() -> String {
    "Please make a guess: ".to_owned()
}

fn format_play_request_context(
    player: &Player,
    hand: &[Card],
    trick: &Trick,
    valid_cards: &Option<HashSet<usize>>,
) -> String {
    const WITH_INDICES: bool = true;
    let hand_string = format_hand(hand, valid_cards, WITH_INDICES);

    let state = format_player_prompt(trick);
    format!("{}: Hand: {hand_string}, {state}", player.name.as_str())
}

fn format_play_request(player: &Player) -> String {
    format!(
        "{}: Select card to play (leftmost is 0): ",
        player.name.as_str()
    )
}

fn format_winners(players: &[Player], winners: &[usize]) -> String {
    let winners_text = winners.iter().map(|i| players[*i].name.as_str()).join(", ");
    format!("The winner(s) is/are {winners_text}!")
}

fn format_request_player_name() -> String {
    "Please input player name: ".to_owned()
}

impl ToString for Message {
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
                valid_cards,
            } => format_play_request_context(player, hand, trick, valid_cards),

            Message::Trick(trick) => format_trick(trick).unwrap_or_default(),

            Message::Scoreboard { state } => format_scoreboard(state),

            Message::Winner(player) => format_winner(player),

            Message::Winners {
                players,
                winner_indices,
            } => format_winners(players, winner_indices),
            Message::RequestPlayerName => format_request_player_name(),
            Message::PlayRequest(player) => format_play_request(player),
            Message::RequestGuess => format_request_guess(),
        }
    }
}
