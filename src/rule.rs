use super::{CardDerection, JokerCard};
use crate::card::*;
use crate::deck::*;
use crate::player::*;
use std::io::{self, BufRead, Write};
use crate::utils::*;

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

        let mut current_player_index = 0;

        // playerが２人以上なら続行
        while self.is_game_continue() {
            println!("＝＝＝＝＝＝＝＝＝＝＝＝ターンスタート");

            let prev_player_index = match self.prev_player_index(current_player_index) {
                Some(player) => player,
                None => current_player_index,
            };

            println!("{}さんのターンです。", self.active_players[current_player_index].get_name());
            let choice_list = self.active_players[current_player_index].choice_list();
            println!("手持ちカードは{:?}", self.active_players[current_player_index].get_cards());

            // prev playerからカードを選択
            println!("{}さんからカードを取ってください. ", self.active_players[prev_player_index].get_name());

            // 選択肢から入力
            println!("=======================");

            let answer = self.choice(&choice_list);
            // prev_playerから選択されたカードをremove
            // ここでバグがある
            let remove_card = self.active_players[prev_player_index].get_cards_mut().remove(answer);

            println!("引いたカードは{:?}", remove_card);

            // current_playerは加えたカードと同じナンバーのカードがあれば捨てる
            match self.active_players[current_player_index].throw_away_pair(remove_card) {
                Some((c1, c2)) => self.pair_to_garbage((c1, c2)),
                None => self.active_players[current_player_index].drow(remove_card),
            }

            // カードがゼロ枚のplayerは上がり
            self.move_finished_players();


            println!("残りのPlayerは{}人です", self.active_players.len());

            // 次の人のターン(current_player=next_player)
            current_player_index = match self.next_player_index(current_player_index) {
                Some(player) => player,
                None => current_player_index,
            };
            println!("＝＝＝＝＝＝＝＝＝＝＝＝ターン終了");

            // break;
        }

        // 勝敗
    }

    fn check_choice(&self, choice_list: &Vec<String>, choice: &String) -> bool {
        let mut valid = false;
        for item in choice_list.iter() {
            if choice == item {
                valid = true;
                break;
            }
        }
        valid
    }

    fn choice(&self, choice_list: &Vec<String>) -> usize {
        let mut answer;
        loop {
            println!("選択肢は{}です ", choice_list.join(", "));
            answer = self.choice_input();
            let valid = self.check_choice(choice_list, &answer);
            if !valid {
                println!("入力値が正しくありません: {}", &answer);
                continue;
            }
            break;
        }
        answer.parse().unwrap()
    }

    fn choice_input(&self) -> String {
        let stdio = io::stdin();
        let input = stdio.lock();
        let output = io::stdout();
        let question = format!("Let's choice");
        let mut answer = prompt(input, output, question);
        answer.retain(|s| s != '\n');
        answer
    }

    // ダブリあれば捨てる、なければドローを汎用化できそう
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
        println!("{}と{}を捨てました", c1.get_info(), c2.get_info());
        c1.set_direction(CardDerection::Front);
        c2.set_direction(CardDerection::Front);
        self.garbage.append(&mut vec![c1, c2]);
    }



    fn next_player_index(&self, index: usize) -> Option<usize> {
        if self.check_exist_player_index(index) {
            // defaultはlast index
            let mut next_index = index + 1;
            if index == self.active_players.len() - 1 {
                next_index = 0;
            }
            Some(next_index)
        } else {
            None
        }
    }

    fn prev_player_index(&self, index: usize) -> Option<usize> {
        if self.check_exist_player_index(index) {
            // defaultはlast index
            let mut prev_index = self.active_players.len() - 1;
            if index != 0 {
                prev_index = index - 1;
            }
            Some(prev_index)
        } else {
            None
        }
    }

    fn check_exist_player(&self, player: &Player) -> bool {
        self.active_players.contains(player)
    }

    fn check_exist_player_index(&self, n: usize) -> bool {
        n < self.active_players.len()
    }

    fn is_game_continue(&self) -> bool {
        self.active_players.len() >= 2
    }

    pub fn get_active_players(&self) -> &Vec<Player> {
        &self.active_players
    }

    pub fn get_active_players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.active_players
    }

    pub fn get_finished_players(&self) -> &Vec<Player> {
        &self.finished_players
    }

    fn move_finished_players(&mut self) -> &Vec<Player> {
        // card check
        let mut finised_players: Vec<Player> = vec![];
        self.active_players.retain(|player| {
            if player.get_cards().len() == 0 {
                // finishに移動
                println!("{}があがりました", player.get_name());
                finised_players.push(Player::new(player.get_name()));
                false
            } else {
                true
            }
        });

        self.finished_players.append(&mut finised_players);

        &self.finished_players
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

    // テスト単位分解する
    // 1, Playerにランダムにデッキのカードを配る
    // 2, リストからのペアの削除
    // #[test]
    // fn without_bubbling_game_setup() {
    //     let players = vec![
    //         Player::new("yugi"),
    //         Player::new("zyounochi"),
    //         Player::new("marik"),
    //     ];
    //     let mut game_master = WithoutBubbling::new(players);
    //     game_master.init();
    //     assert_eq!(game_master.deck.get_cards().len(), 0);
    //     assert!(!game_master.active_players[0].get_cards().is_double_pair());
    //     assert!(!game_master.active_players[1].get_cards().is_double_pair());
    //     assert!(!game_master.active_players[2].get_cards().is_double_pair());
    //     assert!(game_master.active_players[0].get_cards().len() <= 14);
    //     assert!(game_master.active_players[1].get_cards().len() <= 14);
    //     assert!(game_master.active_players[2].get_cards().len() <= 14);
    // }

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
        let mut game_master = WithoutBubbling::new(players);
        assert_eq!(game_master.next_player_index(0), Some(1));
        assert_eq!(game_master.next_player_index(2), Some(0));
        assert_eq!(game_master.next_player_index(4), None);
    }

    #[test]
    fn to_finished_player_from_active_players() {
        let players = vec![
            Player::new("yugi"),
            Player::new("zyounochi"),
            Player::new("marik"),
        ];

        let mut game_master = WithoutBubbling::new(players);
        
        game_master.get_active_players_mut()[2].get_cards_mut().push(Card::new(1, &Mark::Clover));

        assert_eq!(game_master.move_finished_players(), 
            &vec![
                Player::new("yugi"),
                Player::new("zyounochi"),
            ]
        );
        assert_eq!(game_master.get_active_players().len(), 1);
    }

}
