use serde::{Deserialize, Serialize};

#[cfg(test)]
use proptest_derive::Arbitrary;

#[derive(Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Card {
    pub suit: usize,
    pub value: usize,
}
