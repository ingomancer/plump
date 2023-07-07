use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::{iproduct, Itertools};

#[cfg(test)]
use proptest_derive::Arbitrary;
use rand::seq::IteratorRandom;
use serde::Serialize;

use crate::message::Message;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Debug)]
pub struct PlayerName<'a>(&'a str);

impl<'a> PlayerName<'a> {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[derive(Clone, Serialize)]
pub struct Player<'a> {
    pub name: PlayerName<'a>,
    pub human: bool,
    pub hand: Vec<Card>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Debug, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Card {
    pub suit: u32,
    pub value: u32,
}

#[derive(Clone, Serialize)]
pub struct Trick(pub Vec<Card>);

impl Trick {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Clone, Copy, Serialize)]
pub struct PublicState {
    pub guess: Option<u32>,
    pub wins: u32,
    pub score: u32,
}

pub fn create_players(player_names: &Vec<(String, bool)>) -> VecDeque<Player> {
    let mut players = VecDeque::new();
    for (name, human) in player_names {
        players.push_back(Player {
            name: PlayerName(name),
            human: *human,
            hand: Vec::new(),
        });
    }
    players
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn deck_and_hand_size() -> impl Strategy<Value = (HashSet<Card>, u8)> {
        any::<HashSet<Card>>().prop_flat_map(|deck| {
            let len = deck.len();
            (Just(deck), 0..(len + 1) as u8)
        })
    }

    proptest! {
        #[test]
        fn test_create_players(names in any::<Vec<(String, bool)>>()) {
            let players = create_players(&names);
            prop_assert!(players.len() == names.len());
            for ((name, human), player) in names.iter().zip(players.iter()) {
                prop_assert_eq!(PlayerName(name), player.name);
                prop_assert_eq!(human, &player.human);
            }
        }

        #[test]
        fn test_draw_hand((deck, hand_size) in deck_and_hand_size()) {
            let (new_deck, hand) = draw_hand(deck.clone(), hand_size as _);
            prop_assert!(new_deck.is_subset(&deck));
            prop_assert_eq!(hand.len(), hand_size as usize);
            prop_assert_eq!(new_deck.len() + hand_size as usize, deck.len());
            let hand_set: HashSet<Card> = hand.into_iter().collect();
            prop_assert!(hand_set.is_subset(&deck));
        }
    }

    #[test]
    fn test_create_deck() {
        let deck = create_deck();
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_determine_winner() {
        let trick = Trick(vec![
            Card { suit: 0, value: 1 },
            Card { suit: 1, value: 10 },
            Card { suit: 0, value: 7 },
        ]);
        let winner = determine_winner(&trick);
        assert_eq!(winner, 2);
        let trick = Trick(vec![
            Card { suit: 1, value: 1 },
            Card { suit: 1, value: 10 },
            Card { suit: 0, value: 7 },
        ]);
        let winner = determine_winner(&trick);
        assert_eq!(winner, 1);
        let trick = Trick(vec![
            Card { suit: 3, value: 1 },
            Card { suit: 1, value: 10 },
            Card { suit: 0, value: 7 },
        ]);
        let winner = determine_winner(&trick);
        assert_eq!(winner, 0);
        let trick = Trick(vec![
            Card { suit: 0, value: 1 },
            Card { suit: 0, value: 10 },
            Card { suit: 0, value: 11 },
        ]);
        let winner = determine_winner(&trick);
        assert_eq!(winner, 2);
    }
}

pub trait Communicator {
    fn read(&mut self, player: PlayerName, prompt: Message) -> String;
    fn write_to_all(&mut self, text: Message);
    fn write_to_one(&mut self, player: PlayerName, text: Message);
    fn wait_for_reconnect(&mut self, player: &str);
}

pub fn game<C>(communicator: &mut C, players: &mut VecDeque<Player>, num_rounds: u32) -> Vec<usize>
where
    C: Communicator,
{
    let mut down_sets: Vec<u32> = (1..=num_rounds).rev().collect();
    let mut singles: Vec<u32> = (1..players.len() as u32).map(|_| 1).collect();
    let mut up_sets: Vec<u32> = (2..=num_rounds).collect();
    let mut sets = Vec::new();
    sets.append(&mut down_sets);
    sets.append(&mut singles);
    sets.append(&mut up_sets);

    let mut public_state = HashMap::new();
    for player in players.clone() {
        public_state.insert(
            player.name,
            PublicState {
                guess: None,
                wins: 0,
                score: 0,
            },
        );
    }

    for set in sets {
        let mut players_in_set = players.clone();
        let mut deck = create_deck();
        let mut prev_guesses = vec![];
        for player in &mut players_in_set {
            let hand;
            (deck, hand) = draw_hand(deck, set);
            communicator.write_to_all(Message::Turn { whose: player });
            let guess = match player.human {
                true => request_guess(communicator, player, &hand, &prev_guesses, players.len()),
                false => make_guess(&hand, &prev_guesses, players.len()),
            };
            public_state.get_mut(&player.name).unwrap().guess = Some(guess);
            player.hand = hand;
            prev_guesses.push(guess);
        }
        communicator.write_to_all(Message::Guesses {
            state: &public_state,
        });
        let index = determine_start_player(&prev_guesses);
        players_in_set.rotate_left(index);

        while players_in_set
            .front()
            .filter(|p| !p.hand.is_empty())
            .is_some()
        {
            let mut trick = Trick::new();
            for player in &mut players_in_set {
                let hand;
                if player.human {
                    (hand, trick) =
                        play_human_card(communicator, player, player.hand.clone(), trick);
                } else {
                    (hand, trick) = play_card(player.hand.clone(), trick)
                }
                player.hand = hand;
                communicator.write_to_all(Message::Trick(&trick));
            }
            let index = determine_winner(&trick);
            let winner = &players_in_set[index];
            public_state.get_mut(&winner.name).unwrap().wins += 1;
            communicator.write_to_all(Message::Scoreboard {
                state: &public_state,
            });
            communicator.write_to_all(Message::Winner(winner));
            players_in_set.rotate_left(index);
        }
        for player in &players_in_set {
            let player = public_state.get_mut(&player.name).unwrap();
            *player = score_round(*player);
        }
        players.rotate_left(1)
    }

    determine_total_winners(players, &public_state)
}

fn create_deck() -> HashSet<Card> {
    iproduct!(0..4, 0..13)
        .map(|(x, y)| Card { suit: x, value: y })
        .collect()
}

fn draw_hand(deck: HashSet<Card>, num: u32) -> (HashSet<Card>, Vec<Card>) {
    let hand = HashSet::from_iter(
        deck.iter()
            .copied()
            .choose_multiple(&mut rand::thread_rng(), num.try_into().unwrap()),
    );
    (
        deck.difference(&hand).copied().collect(),
        hand.into_iter().sorted().collect(),
    )
}

fn make_guess(hand: &Vec<Card>, guesses: &Vec<u32>, players: usize) -> u32 {
    let mut guess: u32 = hand.iter().filter(|x| x.value >= 7).count() as u32;
    if !validate_guess(hand.len(), guesses, players, guess) {
        let new_guess = hand.iter().filter(|x| x.value >= 9).count() as u32;
        if new_guess == guess {
            guess += 1;
        } else {
            guess = new_guess
        }
    }
    guess
}

fn validate_guess(hand_size: usize, guesses: &Vec<u32>, players: usize, guess: u32) -> bool {
    if guess > hand_size as u32 {
        return false;
    }

    if guesses.len() == players - 1 && (guess + guesses.iter().sum::<u32>()) == hand_size as u32 {
        return false;
    }

    true
}

fn request_guess<C>(
    communicator: &mut C,
    player: &Player,
    hand: &Vec<Card>,
    guesses: &Vec<u32>,
    players: usize,
) -> u32
where
    C: Communicator,
{
    communicator.write_to_one(
        player.name,
        Message::RequestGuessContext {
            player,
            hand,
            guesses,
            players,
        },
    );

    loop {
        let text = communicator.read(player.name, Message::RequestGuess);
        let guess: u32 = match text.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        if validate_guess(hand.len(), guesses, players, guess) {
            return guess;
        }
    }
}

fn determine_start_player(guesses: &[u32]) -> usize {
    guesses
        .iter()
        .position(|x| x == guesses.iter().max().unwrap())
        .unwrap()
}

fn play_human_card<C>(
    communicator: &mut C,
    player: &Player,
    mut hand: Vec<Card>,
    trick: Trick,
) -> (Vec<Card>, Trick)
where
    C: Communicator,
{
    let valid_cards = playable_card_indices(&hand, &trick);

    communicator.write_to_all(Message::Turn { whose: player });

    communicator.write_to_one(
        player.name,
        Message::PlayRequestContext {
            player,
            hand: &hand,
            trick: &trick,
        },
    );

    let Trick(mut cards) = trick;

    loop {
        let text = communicator.read(player.name, Message::PlayRequest(player));

        let index: usize = match text.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if index >= hand.len() {
            continue;
        }

        if let Some(valid) = valid_cards.as_ref() {
            if !valid.contains(&index) {
                continue;
            }
        }

        let card = hand[index];
        hand.retain(|c| *c != card);
        cards.push(card);

        return (hand, Trick(cards.to_vec()));
    }
}

fn play_card(mut hand: Vec<Card>, Trick(mut cards): Trick) -> (Vec<Card>, Trick) {
    let card = hand.iter().choose(&mut rand::thread_rng()).copied();

    if let Some(card) = card {
        hand.retain(|c| *c != card);
        cards.push(card);
    }

    (hand, Trick(cards))
}

pub fn playable_card_indices(hand: &[Card], Trick(cards): &Trick) -> Option<HashSet<usize>> {
    let first_card = match cards.first() {
        Some(card) => card,
        None => return None,
    };

    let indices = HashSet::from_iter(
        hand.iter()
            .enumerate()
            .filter_map(|(index, card)| (card.suit == first_card.suit).then_some(index)),
    );

    (!indices.is_empty()).then_some(indices)
}

fn determine_winner(Trick(cards): &Trick) -> usize {
    let first_card = cards.first().unwrap();
    let first_suit = first_card.suit;

    cards
        .iter()
        .position_max_by_key(|c| if c.suit == first_suit { c.value } else { 0 })
        .unwrap()
}

fn score_round(mut player: PublicState) -> PublicState {
    if let Some(guess) = player.guess.filter(|g| *g == player.wins) {
        player.score += (10 * guess).max(10)
    }

    player.wins = 0;
    player
}

pub type StatePerPlayer<'a> = HashMap<PlayerName<'a>, PublicState>;

fn determine_total_winners(players: &VecDeque<Player>, public: &StatePerPlayer) -> Vec<usize> {
    let mut winners = Vec::new();
    let mut highest_score = u32::MAX;

    for (index, player) in players.iter().enumerate() {
        let player = public.get(&player.name).unwrap();

        match player.score.cmp(&highest_score) {
            Ordering::Greater => {
                highest_score = player.score;
                winners.push(index);
            }
            Ordering::Equal => winners.push(index),
            _ => {}
        };
    }

    winners
}
