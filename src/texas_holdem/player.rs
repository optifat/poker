use crate::card::Card;
use crate::bet;

#[derive(Copy, Clone)]
pub(super) struct Player{
    total_money: u32,
    cards: [Option<Card>; 2],
    current_bet: u32,
}

impl Player{
    pub fn new(total_money: u32) -> Self{
        Player{
            total_money,
            cards: [None; 2],
            current_bet: 0,
        }
    }

    pub fn deal_card(&mut self, card: Card) -> Result<(), std::io::Error>{
        if self.cards[0].is_none(){
            self.cards[0] = Some(card);
            return Ok(());
        }

        if self.cards[1].is_none(){
            self.cards[1] = Some(card);
            return Ok(());
        }

        Err(std::io::Error::new(std::io::ErrorKind::Other, "Player has 2 cards already"))
    }

    pub fn make_bet(&self, previous_bet: u32) -> bet::Bet(){
        println!("Current bet is {}$", u32);
        loop{
            println!("Press 'Enter' to call/check, 'F' to fold or amount of money to raise");
        }
    }
}

impl std::fmt::Display for Player{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "money: {}\n", self.total_money)?;

        for card in self.cards{
            if card.is_none(){
                write!(f, "_ ")?;
                continue
            }
            write!(f, "{} ", card.unwrap())?;
        }

        write!(f, "")
    }
}
