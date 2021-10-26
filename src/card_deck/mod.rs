mod card;

use std::collections::VecDeque;
use self::card::Card;
use rand::prelude::*;

pub struct Deck{
    cards: VecDeque<Card>
}

impl Deck{
    pub fn new() -> Deck{
        let mut deck = VecDeque::with_capacity(52);
        for value in 0..=12{
            for suit in 0..=3{
                deck.push_back(Card::new(value, suit).unwrap());
            }
        }
        Deck{cards: deck}
    }

    pub fn shuffle(&mut self){
        self.cards.make_contiguous().shuffle(&mut rand::thread_rng());
    }
}

impl std::fmt::Display for Deck{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        for card in self.cards.iter(){
            write!(f, "{} ", *card)?;
        }
        write!(f, "")
    }
}
