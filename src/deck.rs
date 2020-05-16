use super::card::*;
use super::{JokerCard, Mark};

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(joker: JokerCard) -> Self {
        let cards = Deck::deck_init(joker);
        Deck { cards }
    }

    pub fn drow(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn drow_multiple(&mut self, n: usize) -> Option<Vec<Card>> {
        let mut drow_cards = vec![];
        for _ in 0..n {
            if let Some(card) = self.cards.pop() {
                drow_cards.push(card);
            } else {
                break;
            }
        }
        Some(drow_cards)
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

    fn deck_init(is_joker: JokerCard) -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];
        for mark in Mark::iterator() {
            cards.append(&mut Deck::make_mark_set(mark));
        }
        match is_joker {
            JokerCard::Zero => cards,
            JokerCard::One => {
                cards.push(Card::new(0, &Mark::Joker));
                cards
            }
            JokerCard::Two => {
                cards.push(Card::new(0, &Mark::Joker));
                cards.push(Card::new(0, &Mark::Joker));
                cards
            }
        }
    }

    fn make_mark_set(mark: &'static Mark) -> Vec<Card> {
        let mut cards: Vec<Card> = vec![];
        for i in 1..14 {
            let card = Card::new(i, mark);
            cards.push(card);
        }
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::Deck;
    use crate::card::*;
    use crate::utils::is_shuffle;
    use crate::{CardDerection, JokerCard, Mark};

    #[test]
    fn make_deck() {
        let deck_not_in_joker_deck = Deck::new(JokerCard::Zero);
        let deck_in_joker_one_deck = Deck::new(JokerCard::One);
        let deck_in_joker_two_deck = Deck::new(JokerCard::Two);

        assert_eq!(deck_not_in_joker_deck.get_cards().len(), 52);
        assert_eq!(deck_in_joker_one_deck.get_cards().len(), 53);
        assert_eq!(deck_in_joker_two_deck.get_cards().len(), 54);
    }

    #[test]
    fn deck_length() {
        let deck = Deck::new(JokerCard::Zero);
        assert_eq!(deck.get_cards().len(), 52);
    }

    #[test]
    fn deck_shuffle() {
        let mut deck = Deck::new(JokerCard::Zero);
        assert!(!is_shuffle(&deck.cards, &Deck::new(JokerCard::Zero).cards));
        deck.get_cards_mut().shuffle();
        assert!(is_shuffle(&deck.cards, &Deck::new(JokerCard::Zero).cards));
    }

    #[test]
    fn one_drow() {
        let mut deck = Deck::new(JokerCard::Zero);
        let drow_card = deck.drow();
        assert_eq!(drow_card, Some(Card::new(13, &Mark::Clover)));
        assert_eq!(deck.cards.len(), 51);
    }

    #[test]
    fn reverse_list() {
        let mut deck = Deck::new(JokerCard::Zero);
        let mut card1 = Card::new(1, &Mark::Clover);
        let mut card2 = Card::new(2, &Mark::Clover);
        let mut card3 = Card::new(3, &Mark::Clover);
        deck.cards = vec![card1, card2, card3];
        deck.get_cards_mut().reverse_direction();
        card1.reverse_direction();
        card2.reverse_direction();
        card3.reverse_direction();
        assert_eq!(deck.get_cards(), &vec![card3, card2, card1]);
        assert!(deck.get_cards().check_cards_direction(CardDerection::Front));
        deck.get_cards_mut().reverse_direction();
        card1.reverse_direction();
        card2.reverse_direction();
        card3.reverse_direction();
        assert_eq!(deck.get_cards(), &vec![card1, card2, card3]);
        assert!(deck.get_cards().check_cards_direction(CardDerection::Back));
    }
}
