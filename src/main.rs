use rust_playing_cards::deck::Deck;
use rust_playing_cards::player::Player;
use rust_playing_cards::rule::WithoutBubbling;
use rust_playing_cards::JokerCard;
use rust_playing_cards::card::*;

fn main() {
    let mut deck = Deck::new(JokerCard::Zero);
    deck.get_cards_mut().shuffle();

    let mut player1 = Player::new("yugi");
    let mut player2 = Player::new("zyounochi");
    let mut player3 = Player::new("marik");
    let mut players = vec![&mut player1, &mut player2, &mut player3];
    WithoutBubbling::init(&mut players);
}
