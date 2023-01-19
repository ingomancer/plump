use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Index,
};

use itertools::iproduct;
use rand::seq::IteratorRandom;

use crate::format::{format_guesses, format_hand};

#[derive(Clone)]
pub struct Player<'a> {
    pub name: &'a str,
    pub human: bool,
    pub hand: Vec<Card>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: u32,
    pub value: u32,
}

pub struct PublicState {
    pub guess: i32,
    pub wins: u32,
    pub score: u32,
}

pub fn create_players(player_names: &Vec<(String, bool)>) -> VecDeque<Player> {
    let mut players = VecDeque::new();
    for (name, human) in player_names {
        players.push_back(Player {
            name: &name,
            human: *human,
            hand: Vec::new(),
        });
    }
    players
}

pub fn game(
    read: fn(&str, &str) -> String,
    write: fn(&str, Option<&str>),
    players: VecDeque<Player>,
    num_rounds: u32,
) {
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
                guess: -1,
                wins: 0,
                score: 0,
            },
        );
    }

    for set in sets {
        let mut players_in_set = players.clone();
        let mut deck = create_deck();
        let mut prev_guesses = vec![];
        for mut player in players_in_set.clone() {
            let hand;
            (deck, hand) = draw_hand(deck, set);
            write(&format!("{}'s turn", player.name), None);
            let guess;
            if player.human {
                guess = request_guess(
                    read,
                    write,
                    player.name,
                    &hand,
                    &prev_guesses,
                    players.len(),
                );
            } else {
                guess = make_guess(&hand, &prev_guesses, players.len());
            }
            public_state.get_mut(player.name).unwrap().guess = guess;
            player.hand = hand;
            prev_guesses.push(guess);
        }
        write(&format_guesses(&public_state), None);
        let index = determine_start_player(&prev_guesses);
        players_in_set.rotate_left(index);
    }
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

fn make_guess(hand: &Vec<Card>, guesses: &Vec<i32>, players: usize) -> i32 {
    let mut guess: i32 = hand.iter().filter(|x| x.value >= 7).count() as i32;
    if !validate_guess(hand.len(), guesses, players, guess) {
        let new_guess = hand.iter().filter(|x| x.value >= 9).count() as i32;
        if new_guess == guess {
            guess += 1;
        } else {
            guess = new_guess
        }
    }
    guess
}

fn validate_guess(hand_size: usize, guesses: &Vec<i32>, players: usize, guess: i32) -> bool {
    if !(0 <= guess && guess <= hand_size as i32) {
        return false;
    } else if guesses.len() == players - 1 {
        if (guess + guesses.iter().sum::<i32>()) == hand_size as i32 {
            return false;
        }
    }
    true
}

fn request_guess(
    read: fn(&str, &str) -> String,
    write: fn(&str, Option<&str>),
    player: &str,
    hand: &Vec<Card>,
    guesses: &Vec<i32>,
    players: usize,
) -> i32 {
    let hand_string = format_hand(hand);
    write(&hand_string, None);
    let mut guess = -1;
    while !validate_guess(hand.len(), guesses, players, guess) {
        guess = read("Please make a guess, ", player)
            .trim()
            .parse()
            .unwrap();
    }
    guess
}

fn determine_start_player(guesses: &Vec<i32>) -> usize {
    guesses
        .iter()
        .position(|x| x == guesses.iter().max().unwrap())
        .unwrap()
}
