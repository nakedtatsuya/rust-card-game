use rust_playing_cards::deck::Deck;
use rust_playing_cards::player::Player;
use rust_playing_cards::rule::WithoutBubbling;
use rust_playing_cards::JokerCard;
use rust_playing_cards::card::*;

fn main() {
    let mut deck = Deck::new(JokerCard::Zero);
    deck.get_cards_mut().shuffle();

    let player1 = Player::new("yugi");
    let player2 = Player::new("zyounochi");
    let player3 = Player::new("marik");
    let players = vec![player1, player2, player3];

    let mut game_master = WithoutBubbling::new(players);

    game_master.init();    
}
