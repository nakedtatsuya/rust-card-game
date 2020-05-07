use crate::card::*;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Player {
            name: name.into(),
            cards: vec![],
        }
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn get_cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }

    pub fn set_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
