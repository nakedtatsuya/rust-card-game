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

    fn shuffle(&mut self);

    fn reverse_direction(&mut self);
}

impl CardList for Vec<Card> {
    fn is_double_pair(&self) -> bool{
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

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.as_mut_slice().shuffle(&mut rng);
    }

    fn reverse_direction(&mut self) {
        self.reverse();
        for i in 0..self.len() {
            self[i].reverse_direction();
        };
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
    use super::Mark;

    #[test]
    fn make_card() {
        let card = Card::new(1, &Mark::Clover);
        assert_eq!(card.get_number(), 1);
        assert_eq!(card.get_mark(), &Mark::Clover);
    }
}
