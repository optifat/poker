mod card_deck;

use crate::card_deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("{}", deck);
}
