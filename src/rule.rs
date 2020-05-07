use super::JokerCard;
use crate::card::*;
use crate::deck::*;
use crate::player::*;

trait Rule {}

pub struct WithoutBubbling;

impl WithoutBubbling {
    pub fn init(players: &mut Vec<&mut Player>) {
        let mut deck = Deck::new(JokerCard::One);
        deck.get_cards_mut().shuffle();
        for i in 0..deck.get_cards().len() {
            let index = i % players.len();
            let cards = players[index].get_cards_mut();
            match deck.drow() {
                Some(card) => match WithoutBubbling::check_pair(cards, &card) {
                    Some(target) => &WithoutBubbling::throw_away_pair(cards, &target),
                    None => &players[index].get_cards_mut().push(card),
                },
                None => &(),
            };
        }

        // 計算量は少ないけどデッキの所有権が不自然。まとめてpop()できるメソッドあるのか？
        // let mut cards: Vec<Vec<Card>> = Vec::new();
        // let split_point = deck.get_cards().len() / players.len();
        // let surplus = deck.get_cards().len() % players.len();
        // let mut start_point = 0;
        // for i in 0..players.len() {
        //     let mut end_point = start_point+split_point;
        //     if i < surplus {
        //         end_point += 1;
        //     }
        //     let mut split_cards: Vec<Card> = deck.get_cards_mut()[start_point..end_point].to_vec();
        //     &players[i].get_cards_mut().append(&mut split_cards);
        //     cards.push(split_cards);
        //     start_point += split_point;
        // }
    }

    fn check_pair(cards: &Vec<Card>, new_card: &Card) -> Option<Card> {
        let mut result = None;
        for card in cards.iter() {
            if card.is_equal_number(new_card) {
                let c1 = Card::new(card.get_number(), card.get_mark());
                result = Some(c1);
            }
        }
        result
    }

    fn throw_away_pair(cards: &mut Vec<Card>, target: &Card) {
        cards.retain(|card| card != target);
    }
}

#[cfg(test)]
mod tests {
    use super::WithoutBubbling;
    use crate::card::*;
    use crate::player::*;
    use crate::Mark;

    #[test]
    fn without_bubbling_game_setup() {
        let mut player1 = Player::new("yugi");
        let mut player2 = Player::new("zyounochi");
        let mut player3 = Player::new("marik");
        let mut players = vec![&mut player1, &mut player2, &mut player3];
        WithoutBubbling::init(&mut players);
        assert!(!is_double_pair(player1.get_cards()));
        assert!(!is_double_pair(player2.get_cards()));
        assert!(!is_double_pair(player3.get_cards()));
        assert!(player1.get_cards().len() <= 14);
        assert!(player2.get_cards().len() <= 14);
        assert!(player3.get_cards().len() <= 14);
    }

    fn is_double_pair(cards: &Vec<Card>) -> bool {
        let mut result = false;
        for i in 0..cards.len() {
            for x in (i + 1)..cards.len() {
                if cards[i].get_number() == cards[x].get_number() {
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

    #[test]
    fn double_checker_by_list() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let card4 = Card::new(2, &Mark::Diamond);
        let cards = vec![card1, card2, card3];
        assert_eq!(is_double_pair(&cards), false);
        let cards = vec![card1, card2, card3, card4];
        assert_eq!(is_double_pair(&cards), true);
    }

    #[test]
    fn check_pair_true() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let cards = vec![card1, card2, card3];
        let drow_card = Card::new(2, &Mark::Diamond);
        assert_eq!(WithoutBubbling::check_pair(&cards, &drow_card), Some(card2));
    }

    #[test]
    fn check_pair_false() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let cards = vec![card1, card2, card3];
        let drow_card = Card::new(4, &Mark::Diamond);
        assert_eq!(WithoutBubbling::check_pair(&cards, &drow_card), None);
    }

    #[test]
    fn double_to_gabage() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let target_card = Card::new(2, &Mark::Clover);
        let mut cards = vec![card1, card2, card3];
        WithoutBubbling::throw_away_pair(&mut cards, &target_card);
        assert_eq!(cards, vec![card1, card3]);
    }
}
