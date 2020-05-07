use super::JokerCard;
use crate::card::*;
use crate::deck::*;
use crate::player::*;

trait Rule {}

pub struct WithoutBubbling {
    active_players: Vec<Player>,
    deck: Deck,
    garbage: Vec<Card>,
    finished_players: Vec<Player>,
}

impl WithoutBubbling {

    pub fn new(active_players: Vec<Player>) -> Self {
        WithoutBubbling {
            active_players,
            deck: Deck::new(JokerCard::One),
            garbage: Default::default(),
            finished_players:Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.deck.get_cards_mut().shuffle();
        for i in 0..self.deck.get_cards().len() {
            let index = i % self.active_players.len();
            match self.deck.drow() {
                Some(drow_card) => self.throw_away_pair(index, drow_card),
                None => (),
            };
        }
    }
    
    fn throw_away_pair(&mut self, player_index: usize, drow_card: Card) {
        let cards = self.active_players[player_index].get_cards_mut();
        if cards.check_pair(&drow_card) {
            match cards.throw_away_pair(drow_card) {
                Some((c1, c2)) => {
                    self.pair_to_garbage((c1, c2));
                },
                None => (),       
            }
        } else {
            cards.push(drow_card);
        }
    }

    fn pair_to_garbage(&mut self, set_cards: (Card, Card)) {
        let (c1, c2) = set_cards;
        self.garbage.append(&mut vec![c1, c2]);
    }
    
}

#[cfg(test)]
mod tests {
    use super::WithoutBubbling;
    use crate::card::*;
    use crate::player::*;
    use crate::Mark;

    #[test]
    fn new_game() {
        let player1 = Player::new("yugi");
        let player2 = Player::new("zyounochi");
        let player3 = Player::new("marik");
        let players = vec![player1, player2, player3];
        let game_master = WithoutBubbling::new(players);
        assert_eq!(game_master.active_players.len(), 3);
        assert_eq!(game_master.garbage.len(), 0);
        assert_eq!(game_master.finished_players.len(), 0);
        assert_eq!(game_master.deck.get_cards().len(), 53);
    }

    #[test]
    fn without_bubbling_game_setup() {
        let player1 = Player::new("yugi");
        let player2 = Player::new("zyounochi");
        let player3 = Player::new("marik");
        let players = vec![player1, player2, player3];

        let mut game_master = WithoutBubbling::new(players);

        game_master.init();    
        assert_eq!(game_master.deck.get_cards().len(), 0);
        assert!(!is_double_pair(game_master.active_players[0].get_cards()));
        assert!(!is_double_pair(game_master.active_players[1].get_cards()));
        assert!(!is_double_pair(game_master.active_players[2].get_cards()));
        assert!(game_master.active_players[0].get_cards().len() <= 14);
        assert!(game_master.active_players[1].get_cards().len() <= 14);
        assert!(game_master.active_players[2].get_cards().len() <= 14);
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
        assert_eq!(cards.check_pair(&drow_card), true);
    }

    #[test]
    fn check_pair_false() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let cards = vec![card1, card2, card3];
        let drow_card = Card::new(4, &Mark::Diamond);
        assert_eq!(cards.check_pair(&drow_card), false);
    }

    #[test]
    fn double_to_gabage() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let target_card = Card::new(2, &Mark::Diamond);
        let cards = vec![card1, card2, card3];


        let player1 = Player::new("yugi");
        let players = vec![player1];
        let mut game_master = WithoutBubbling::new(players);
        game_master.active_players[0].set_cards(cards);


        game_master.throw_away_pair(0, target_card);
        assert_eq!(game_master.active_players[0].get_cards(), &vec![card1, card3]);
        assert_eq!(game_master.garbage, vec![card2, target_card]);

    }
}
