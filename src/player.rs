use crate::card::*;
use crate::rule::*;
use crate::utils::*;

use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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

    pub fn new_players(n: usize) -> Vec<Player> {
        let mut players: Vec<Player> = Vec::new();
        let mut i = 0;
        while i < n {
            let stdio = io::stdin();
            let input = stdio.lock();
            let output = io::stdout();
            let question = format!("Enter the name of player{}", i + 1);
            let name = Player::input_player_name(input, output, question);
            match Player::check_name_length(&name) {
                Ok(()) => {
                    players.push(Player::new(name));
                    i += 1;
                }
                Err(s) => {
                    println!("{}", s);
                }
            }
        }
        players
    }

    pub fn throw_away_pair(&mut self, drow_card: Card) -> Option<(Card, Card)> {
        match self.get_cards_mut().throw_away_pair(drow_card) {
            Some((c1, c2)) => Some((c1, c2)),
            None => None,
        }
    }

    pub fn drow(&mut self, drow_card: Card) {
        self.get_cards_mut().push(drow_card);
    }

    fn input_player_name<R, W>(reader: R, writer: W, question: impl Into<String>) -> String
    where
        R: BufRead,
        W: Write,
    {
        let mut answer = prompt(reader, writer, format!("{}", question.into()));
        answer.retain(|s| s != '\n');
        answer
    }

    fn check_name_length(name: &String) -> Result<(), String> {
        if name.len() > 20 {
            Err(format!(
                "Validate: name length is expected under 20. name: {}",
                name
            ))
        } else if name.is_empty() {
            Err(format!("Validate: name is empty"))
        } else {
            Ok(())
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

    pub fn choice_list(&self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();

        for n in 0..self.cards.len() {
            list.push(n.to_string());
        }
        list
    }
}

mod tests {
    use super::Player;
    use crate::card::*;
    use crate::Mark;
    #[test]
    fn next_player_card_choice() {
        let mut player = Player::new("yugi");
        let cards = vec![
            Card::new(5, &Mark::Clover),
            Card::new(2, &Mark::Clover),
            Card::new(3, &Mark::Clover),
        ];
        player.set_cards(cards);

        assert_eq!(player.choice_list(), vec!["0", "1", "2"]);
    }

    #[test]
    fn input_player_name() {
        let input = b"yugi";
        let mut output = Vec::new();

        let answer = Player::input_player_name(
            &input[..],
            &mut output,
            format!("Enter the name of player1"),
        );

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Enter the name of player1\n");
        assert_eq!(answer, "yugi");
    }

    #[test]
    fn invalid_input_name_length_over20() {
        let name = "abcdeabcdeabcdeabcdea".to_string();
        let result = Player::check_name_length(&name);
        assert!(result.is_err());
    }

    #[test]
    fn invalid_input_name_empty() {
        let name = "".to_string();
        let result = Player::check_name_length(&name);
        assert!(result.is_err());
    }

    #[test]
    fn valid_input_name_length_under20() {
        let name = "abcdeabcdeabcdeabcde".to_string();
        let result = Player::check_name_length(&name);
        assert!(result.is_ok());
    }
}
