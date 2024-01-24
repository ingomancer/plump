use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use rand::seq::IteratorRandom;

use crate::structs::Card;

pub type Deck = HashSet<Card>;

pub fn create_deck() -> Deck {
    iproduct!(0..4, 0..13)
        .map(|(x, y)| Card { suit: x, value: y })
        .collect()
}

pub fn draw_hand(deck: Deck, num: usize) -> (Deck, Vec<Card>) {
    let hand = HashSet::from_iter(
        deck.iter()
            .copied()
            .choose_multiple(&mut rand::thread_rng(), num),
    );
    (
        deck.difference(&hand).copied().collect(),
        hand.into_iter().sorted().collect(),
    )
}
#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn deck_and_hand_size() -> impl Strategy<Value = (Deck, usize)> {
        any::<Deck>().prop_flat_map(|deck| {
            let len = deck.len();
            (Just(deck), 0..=len)
        })
    }

    proptest! {
        #[test]
        fn test_draw_hand((deck, hand_size) in deck_and_hand_size()) {
            let (new_deck, hand) = draw_hand(deck.clone(), hand_size);
            prop_assert!(new_deck.is_subset(&deck));
            prop_assert_eq!(hand.len(), hand_size);
            prop_assert_eq!(new_deck.len() + hand_size, deck.len());
            let hand_set: Deck = hand.into_iter().collect();
            prop_assert!(hand_set.is_subset(&deck));
        }
    }

    #[test]
    fn test_create_deck() {
        let deck = create_deck();
        assert_eq!(deck.len(), 52);
    }
}
