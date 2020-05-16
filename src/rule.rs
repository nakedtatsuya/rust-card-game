use super::{CardDerection, JokerCard};
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
            finished_players: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.deck.get_cards_mut().shuffle();
        self.active_players.shuffle();
        self.hand_out_cards();

        // 順番出力
        println!("順番はこちら");
        for (i, player) in self.get_active_players().iter().enumerate() {
            println!("{}番目は{}さん", i + 1, player.get_name());
        }

        let mut current_player = &self.active_players[0];

        // playerが２人以上なら続行
        while self.is_game_continue() {
            let prev_player = match self.prev_player(&current_player) {
                Some(player) => player,
                None => current_player,
            };

            println!("{}さんのターンです。", current_player.get_name());
            let choice_list = current_player.choice_list();

            // prev playerからカードを選択
            println!("{}さんからカードを取ってください. ", prev_player.get_name());
            println!("選択肢は{}です ", choice_list.join(", "));

            // 選択肢から入力


            // prev_playerから選択されたカードをremove


            // current_playerは加えたカードと同じナンバーのカードがあれば捨てる

            // カードがゼロ枚のplayerは上がり

            // 次の人のターン(current_player=next_player)
            current_player = match self.next_player(&current_player) {
                Some(player) => player,
                None => current_player,
            };

            break;
        }

        // 勝敗
    }

    fn hand_out_cards(&mut self) {
        for i in 0..self.deck.get_cards().len() {
            let index = i % self.active_players.len();
            match self.deck.drow() {
                Some(drow_card) => match self.active_players[index].throw_away_pair(drow_card) {
                    Some((c1, c2)) => self.pair_to_garbage((c1, c2)),
                    None => self.active_players[index].drow(drow_card),
                },
                None => (),
            };
        }
    }

    pub fn pair_to_garbage(&mut self, set_cards: (Card, Card)) {
        let (mut c1, mut c2) = set_cards;
        c1.set_direction(CardDerection::Front);
        c2.set_direction(CardDerection::Front);
        self.garbage.append(&mut vec![c1, c2]);
    }

    fn next_player(&self, player: &Player) -> Option<&Player> {
        if self.check_exist_player(player) {
            let mut current_index = 0;
            for (index, elem) in self.active_players.iter().enumerate() {
                if elem == player {
                    current_index = index;
                    break;
                }
            }

            let mut next_index = 0;
            if current_index != self.active_players.len() - 1 {
                next_index = current_index + 1;
            }

            self.active_players.get(next_index)
        } else {
            None
        }
    }

    fn prev_player(&self, player: &Player) -> Option<&Player> {
        if self.check_exist_player(player) {
            let mut current_index = 0;
            for (index, elem) in self.active_players.iter().enumerate() {
                if elem == player {
                    current_index = index;
                    break;
                }
            }

            let mut next_index = self.active_players.len() - 1;
            if current_index != 0 {
                next_index = current_index - 1;
            }

            self.active_players.get(next_index)
        } else {
            None
        }
    }

    fn check_exist_player(&self, player: &Player) -> bool {
        self.active_players.contains(player)
    }

    fn is_game_continue(&self) -> bool {
        self.active_players.len() >= 2
    }

    pub fn get_active_players(&self) -> &Vec<Player> {
        &self.active_players
    }
}

#[cfg(test)]
mod tests {
    use super::WithoutBubbling;
    use crate::card::*;
    use crate::player::*;
    use crate::{CardDerection, Mark};

    #[test]
    fn new_game() {
        let players = vec![
            Player::new("yugi"),
            Player::new("zyounochi"),
            Player::new("marik"),
        ];
        let game_master = WithoutBubbling::new(players);
        assert_eq!(game_master.active_players.len(), 3);
        assert_eq!(game_master.garbage.len(), 0);
        assert_eq!(game_master.finished_players.len(), 0);
        assert_eq!(game_master.deck.get_cards().len(), 53);
    }

    #[test]
    fn without_bubbling_game_setup() {
        let players = vec![
            Player::new("yugi"),
            Player::new("zyounochi"),
            Player::new("marik"),
        ];
        let mut game_master = WithoutBubbling::new(players);
        game_master.init();
        assert_eq!(game_master.deck.get_cards().len(), 0);
        assert!(!game_master.active_players[0].get_cards().is_double_pair());
        assert!(!game_master.active_players[1].get_cards().is_double_pair());
        assert!(!game_master.active_players[2].get_cards().is_double_pair());
        assert!(game_master.active_players[0].get_cards().len() <= 14);
        assert!(game_master.active_players[1].get_cards().len() <= 14);
        assert!(game_master.active_players[2].get_cards().len() <= 14);
    }

    #[test]
    fn double_checker_by_list() {
        let card1 = Card::new(1, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Clover);
        let card3 = Card::new(3, &Mark::Clover);
        let card4 = Card::new(2, &Mark::Diamond);
        let cards = vec![card1, card2, card3];
        assert_eq!(cards.is_double_pair(), false);
        let cards = vec![card1, card2, card3, card4];
        assert_eq!(cards.is_double_pair(), true);
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
        let card1 = Card::new(2, &Mark::Clover);
        let card2 = Card::new(2, &Mark::Diamond);
        let players = vec![Player::new("yugi")];
        let mut game_master = WithoutBubbling::new(players);
        game_master.pair_to_garbage((card1, card2));
        assert_eq!(game_master.garbage[0].get_number(), 2);
        assert_eq!(game_master.garbage[0].get_mark(), &Mark::Clover);
        assert_eq!(game_master.garbage[0].get_direction(), CardDerection::Front);
        assert_eq!(game_master.garbage[1].get_number(), 2);
        assert_eq!(game_master.garbage[1].get_mark(), &Mark::Diamond);
        assert_eq!(game_master.garbage[1].get_direction(), CardDerection::Front);
    }

    #[test]
    fn card_choice() {}

    #[test]
    fn loop_until_game_ends() {
        let players = vec![
            Player::new("yugi"),
            Player::new("zyounochi"),
            Player::new("marik"),
        ];
        let game_master = WithoutBubbling::new(players);
        assert_eq!(game_master.is_game_continue(), true);
        let player1 = Player::new("yugi");
        let players = vec![player1];
        let game_master = WithoutBubbling::new(players);
        assert_eq!(game_master.is_game_continue(), false);
    }

    #[test]
    fn next_player() {
        let players = vec![
            Player::new("yugi"),
            Player::new("zyounochi"),
            Player::new("marik"),
        ];
        let game_master = WithoutBubbling::new(players);
        let player1 = Player::new("yugi");
        let player2 = Player::new("zyounochi");
        let player3 = Player::new("marik");
        assert_eq!(game_master.next_player(&player1), Some(&player2));
        assert_eq!(game_master.next_player(&player3), Some(&player1));
        let none_player = Player::new("none");
        assert_eq!(game_master.next_player(&none_player), None);
    }
}
