use itertools::Itertools;
use playing_cards::{
    helpers::{create_deck, draw_hand},
    structs::Card,
};
use rand::seq::IteratorRandom;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use protocol::{
    message::Message,
    structs::{Player, PlayerName, PublicState, StatePerPlayer, Trick},
};

pub fn create_players(player_names: Vec<(String, bool)>) -> VecDeque<Player> {
    let mut players = VecDeque::new();
    for (name, human) in player_names {
        players.push_back(Player {
            name: PlayerName(name),
            human,
            hand: Vec::new(),
        });
    }
    players
}

pub trait Communicator {
    fn read(&mut self, player: &PlayerName, prompt: Message) -> String;
    fn write_to_all(&mut self, text: Message);
    fn write_to_one(&mut self, player: &PlayerName, text: Message);
    fn wait_for_reconnect(&mut self, player: &str);
}

pub async fn game<C>(communicator: &mut C, mut players: VecDeque<Player>, num_rounds: usize)
where
    C: Communicator,
{
    let mut down_sets: Vec<usize> = (1..=num_rounds).rev().collect();
    let mut singles: Vec<usize> = (1..players.len()).map(|_| 1).collect();
    let mut up_sets: Vec<usize> = (2..=num_rounds).collect();
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
            communicator.write_to_all(Message::Turn {
                whose: player.clone(),
            });
            let guess = if player.human {
                request_guess(communicator, player, &hand, &prev_guesses, players.len())
            } else {
                make_guess(&hand, &prev_guesses, players.len())
            };
            public_state.get_mut(&player.name).unwrap().guess = Some(guess);
            player.hand = hand;
            prev_guesses.push(guess);
        }
        communicator.write_to_all(Message::Guesses {
            state: public_state.clone(),
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
                    (hand, trick) = play_card(player.hand.clone(), trick);
                }
                player.hand = hand;
                communicator.write_to_all(Message::Trick(trick.clone()));
            }
            let index = determine_winner(&trick);
            let winner = &players_in_set[index];
            public_state.get_mut(&winner.name).unwrap().wins += 1;
            communicator.write_to_all(Message::Scoreboard {
                state: public_state.clone(),
            });
            communicator.write_to_all(Message::Winner(winner.clone()));
            players_in_set.rotate_left(index);
        }
        for player in &players_in_set {
            let player = public_state.get_mut(&player.name).unwrap();
            *player = score_round(*player);
        }
        players.rotate_left(1);
    }
    let winners = determine_total_winners(&players, &public_state);

    let players_vec: Vec<Player> = players.into_iter().collect_vec();
    communicator.write_to_all(Message::Winners {
        players: players_vec,
        winner_indices: winners,
    });
}

fn make_guess(hand: &Vec<Card>, guesses: &Vec<usize>, players: usize) -> usize {
    let mut guess = hand.iter().filter(|x| x.value >= 7).count();
    if !validate_guess(hand.len(), guesses, players, guess) {
        let new_guess = hand.iter().filter(|x| x.value >= 9).count();
        if new_guess == guess {
            guess += 1;
        } else {
            guess = new_guess;
        }
    }
    guess
}

fn validate_guess(hand_size: usize, guesses: &Vec<usize>, players: usize, guess: usize) -> bool {
    if guess > hand_size {
        return false;
    }

    if guesses.len() == players - 1 && (guess + guesses.iter().sum::<usize>()) == hand_size {
        return false;
    }

    true
}

fn request_guess<C>(
    communicator: &mut C,
    player: &Player,
    hand: &Vec<Card>,
    guesses: &Vec<usize>,
    players: usize,
) -> usize
where
    C: Communicator,
{
    communicator.write_to_one(
        &player.name,
        Message::RequestGuessContext {
            player: player.clone(),
            hand: hand.clone(),
            guesses: guesses.clone(),
            players,
        },
    );

    loop {
        let text = communicator.read(&player.name, Message::RequestGuess);
        let Ok(guess) = text.trim().parse() else {
            continue;
        };

        if validate_guess(hand.len(), guesses, players, guess) {
            return guess;
        }
    }
}

fn determine_start_player(guesses: &[usize]) -> usize {
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

    communicator.write_to_all(Message::Turn {
        whose: player.clone(),
    });

    communicator.write_to_one(
        &player.name,
        Message::PlayRequestContext {
            player: player.clone(),
            hand: hand.clone(),
            trick: trick.clone(),
            valid_cards: valid_cards.clone(),
        },
    );

    let Trick(mut cards) = trick;

    loop {
        let text = communicator.read(&player.name, Message::PlayRequest(player.clone()));

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

        return (hand, Trick(cards.clone()));
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
        player.score += (10 * guess).max(10);
    }

    player.wins = 0;
    player
}

fn determine_total_winners(players: &VecDeque<Player>, public: &StatePerPlayer) -> Vec<usize> {
    let mut winners = Vec::new();
    let mut highest_score = usize::MIN;

    for (index, player) in players.iter().enumerate() {
        let player = public.get(&player.name).unwrap();

        match player.score.cmp(&highest_score) {
            Ordering::Greater => {
                highest_score = player.score;
                winners.clear();
                winners.push(index);
            }
            Ordering::Equal => winners.push(index),
            Ordering::Less => {}
        };
    }

    winners
}

fn playable_card_indices(hand: &[Card], Trick(cards): &Trick) -> Option<HashSet<usize>> {
    let first_card = cards.first()?;

    let indices = hand
        .iter()
        .enumerate()
        .filter_map(|(index, card)| (card.suit == first_card.suit).then_some(index))
        .collect::<HashSet<_>>();

    (!indices.is_empty()).then_some(indices)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use protocol::structs::Trick;

    proptest! {
        #[test]
        fn test_create_players(names in any::<Vec<(String, bool)>>()) {
            let players = create_players(names.clone());
            prop_assert!(players.len() == names.len());
            for ((name, human), player) in names.iter().zip(players.iter()) {
                prop_assert_eq!(PlayerName(name.to_string()), player.name.clone());
                prop_assert_eq!(human, &player.human);
            }
        }
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
