use self::Mark::*;
use std::slice::Iter;

pub mod card;
pub mod deck;
pub mod player;
pub mod rule;
pub mod utils;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
pub enum Mark {
    Heart,
    Diamond,
    Spade,
    Clover,
    Joker,
}

impl Mark {
    pub fn iterator() -> Iter<'static, Mark> {
        static MARKS: [Mark; 4] = [Heart, Diamond, Spade, Clover];
        MARKS.iter()
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord, Clone, Copy)]
pub enum CardDerection {
    Front,
    Back,
}

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
pub enum JokerCard {
    Zero,
    One,
    Two,
}
