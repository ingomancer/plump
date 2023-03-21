use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::{iproduct, Itertools};

#[cfg(test)]
use proptest_derive::Arbitrary;
use rand::seq::IteratorRandom;

use crate::message::Message;

#[derive(Clone)]
pub struct Player<'a> {
    pub name: &'a str,
    pub human: bool,
    pub hand: Vec<Card>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Debug)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Card {
    pub suit: u32,
    pub value: u32,
}

#[derive(Clone)]
pub struct Trick(pub Vec<Card>);

impl Trick {
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Clone, Copy)]
pub struct PublicState {
    pub guess: Option<u32>,
    pub wins: u32,
    pub score: u32,
}

pub fn create_players(player_names: &Vec<(String, bool)>) -> VecDeque<Player> {
    let mut players = VecDeque::new();
    for (name, human) in player_names {
        players.push_back(Player {
            name,
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

    proptest! {
        #[test]
        fn test_create_players(names in any::<Vec<(String, bool)>>()) {
            let players = create_players(&names);
            prop_assert!(players.len() == names.len());
            for ((name, human), player) in names.iter().zip(players.iter()) {
                prop_assert!(name == player.name);
                prop_assert!(human == &player.human);
            }
        }

        #[test]
        fn test_draw_hand(deck in any::<HashSet<Card>>(), hand_size in any::<u8>()) {
            prop_assume!(deck.len() >= hand_size as _);
            let (new_deck, hand) = draw_hand(deck.clone(), hand_size as _);
            prop_assert!(new_deck.is_subset(&deck));
            prop_assert!(hand.len() == hand_size as _ );
            prop_assert!(new_deck.len() + hand_size as usize == deck.len());
            let hand_set: HashSet<Card> = hand.into_iter().collect();
            prop_assert!(hand_set.is_subset(&deck));

        }

    }

    #[test]
    fn test_create_deck() {
        let deck = create_deck();
        assert!(deck.len() == 52);
    }
}

pub trait Communicator {
    fn read(&mut self, prompt: &str, player: &str) -> String;
    fn write(&mut self, text: Message, player: Option<&str>);
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
            communicator.write(Message::Turn { whose: player }, None);
            let guess = match player.human {
                true => request_guess(communicator, player, &hand, &prev_guesses, players.len()),
                false => make_guess(&hand, &prev_guesses, players.len()),
            };
            public_state.get_mut(player.name).unwrap().guess = Some(guess);
            player.hand = hand;
            prev_guesses.push(guess);
        }
        communicator.write(
            Message::Guesses {
                state: &public_state,
            },
            None,
        );
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
                communicator.write(Message::Trick(&trick), None);
            }
            let index = determine_winner(&trick);
            let winner = &players_in_set[index];
            public_state.get_mut(winner.name).unwrap().wins += 1;
            communicator.write(
                Message::Scoreboard {
                    state: &public_state,
                },
                None,
            );
            communicator.write(Message::Winner(winner), None);
            players_in_set.rotate_left(index);
        }
        for player in &players_in_set {
            let player = public_state.get_mut(player.name).unwrap();
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
        hand.into_iter().collect(),
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
    let text = Message::RequestGuessContext {
        player,
        hand,
        guesses,
        players,
    };

    communicator.write(text, Some(player.name));

    loop {
        let text = communicator.read("Please make a guess: ", player.name);
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
    hand = hand.into_iter().sorted().collect();
    let valid_cards = playable_card_indices(&hand, &trick);

    let turn_string = Message::Turn { whose: player };
    communicator.write(turn_string, None);

    let request_context = Message::PlayRequestContext {
        player,
        hand: &hand,
        trick: &trick,
    };

    communicator.write(request_context, Some(player.name));

    let Trick(mut cards) = trick;

    loop {
        let text = communicator.read(
            &format!("{}: Select card to play (leftmost is 0): ", &player.name),
            player.name,
        );

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

        return (hand, Trick(cards));
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
        .filter(|c| c.suit == first_suit)
        .position_max_by_key(|c| c.value)
        .unwrap()
}

fn score_round(mut player: PublicState) -> PublicState {
    if let Some(guess) = player.guess.filter(|g| *g == player.wins) {
        player.score += (10 * guess).max(10)
    }

    player.wins = 0;
    player
}

fn determine_total_winners(
    players: &VecDeque<Player>,
    public: &HashMap<&str, PublicState>,
) -> Vec<usize> {
    let mut winners = Vec::new();
    let mut highest_score = u32::MAX;

    for (index, player) in players.iter().enumerate() {
        let player = public.get(player.name).unwrap();

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
