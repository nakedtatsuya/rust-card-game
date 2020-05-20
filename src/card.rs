use super::{CardDerection, Mark};
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Card {
    number: u8,
    mark: &'static Mark,
    direction: CardDerection,
}

pub trait CardList {
    fn is_double_pair(&self) -> bool;

    fn check_cards_direction(&self, direction: CardDerection) -> bool;

    // fn shuffle(&mut self);

    fn reverse_direction(&mut self);

    fn throw_away_pair(&mut self, target: Card) -> Option<(Card, Card)>;

    fn check_pair(&self, new_card: &Card) -> bool;
}

pub trait Shuffle {
    fn shuffle(&mut self);
}

impl<T> Shuffle for Vec<T> {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.as_mut_slice().shuffle(&mut rng);
    }
}

impl CardList for Vec<Card> {
    fn is_double_pair(&self) -> bool {
        let mut result = false;
        for i in 0..self.len() {
            for x in (i + 1)..self.len() {
                if self[i].get_number() == self[x].get_number() {
                    result = true;
                    break;
                }
            }
            if result {
                break;
            }
        }
        result
    }

    fn check_cards_direction(&self, direction: CardDerection) -> bool {
        let mut result = true;
        for i in 0..self.len() {
            if self[i].get_direction() != direction {
                result = false;
            }
        }
        result
    }

    fn reverse_direction(&mut self) {
        self.reverse();
        for i in 0..self.len() {
            self[i].reverse_direction();
        }
    }

    fn throw_away_pair(&mut self, drow_card: Card) -> Option<(Card, Card)> {
        let mut set: Option<(Card, Card)> = None;
        self.retain(|&card| {
            if card.get_number() == drow_card.get_number() {
                set = Some((card, drow_card));
                false
            } else {
                true
            }
        });
        set
    }

    fn check_pair(&self, new_card: &Card) -> bool {
        let mut result = false;
        for card in self.iter() {
            if card.is_equal_number(new_card) {
                result = true;
                break;
            }
        }
        result
    }
}


impl Card {
    pub fn new(number: u8, mark: &'static Mark) -> Self {
        Card {
            number,
            mark,
            direction: CardDerection::Back,
        }
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn get_direction(&self) -> CardDerection {
        self.direction
    }

    pub fn set_direction(&mut self, direction: CardDerection) {
        self.direction = direction;
    }

    pub fn reverse_direction(&mut self) {
        match self.direction {
            CardDerection::Front => self.direction = CardDerection::Back,
            CardDerection::Back => self.direction = CardDerection::Front,
        };
    }

    pub fn get_mark(&self) -> &'static Mark {
        self.mark
    }

    pub fn is_equal_number(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[cfg(test)]
mod tests {
    use super::Card;
    use super::CardList;
    use super::Mark;

    #[test]
    fn make_card() {
        let card = Card::new(1, &Mark::Clover);
        assert_eq!(card.get_number(), 1);
        assert_eq!(card.get_mark(), &Mark::Clover);
    }

    #[test]
    fn check_pair_number() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let drow_card = Card::new(2, &Mark::Clover);
        let mut cards = vec![card1, card2, card3];

        assert_eq!(cards.throw_away_pair(drow_card), Some((card2, drow_card)));
        assert_eq!(cards, vec![card1, card3]);
    }
}
