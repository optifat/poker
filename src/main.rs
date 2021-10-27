mod card_deck;
mod card;
mod bet;
mod texas_holdem;

use crate::card_deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("{}", deck);
}
