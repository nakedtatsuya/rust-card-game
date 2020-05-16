use rust_playing_cards::card::*;
use rust_playing_cards::deck::Deck;
use rust_playing_cards::player::Player;
use rust_playing_cards::rule::WithoutBubbling;
use rust_playing_cards::JokerCard;

fn main() {
    let mut deck = Deck::new(JokerCard::Zero);
    deck.get_cards_mut().shuffle();
    let players = Player::new_players(3);

    let mut game_master = WithoutBubbling::new(players);
    game_master.init();
}
