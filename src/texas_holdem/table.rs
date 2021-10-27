use crate::card::Card;
use super::player::Player;

pub(super) struct Table{
    community_cards: [Option<Card>; 5],
    bank: u32,
    players: [Option<Player>; 10],
    dealer_index: usize,
    big_blind: u32,
}

impl Table{
    pub fn new(big_blind: u32) -> Self{
        Table{
            community_cards: [None; 5],
            bank: 0,
            players: [None; 10],
            dealer_index: 0,
            big_blind
        }
    }

    pub fn deal_card(&mut self, card: Card) -> Result<(), std::io::Error>{
        for mut community_card in self.community_cards{
            if community_card.is_some(){
                continue;
            }
            community_card = Some(card);
            return Ok(());
        }

        Err(std::io::Error::new(std::io::ErrorKind::Other, "There are 5 community cards already"))
    }

    pub fn get_players(&self) -> &[Option<Player>; 10]{
        &self.players
    }
}

impl std::fmt::Display for Table{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "bank: {}\n", self.bank)?;

        for card in self.community_cards{
            if card.is_none(){
                write!(f, "_ ")?;
                continue;
            }
            write!(f, "{}", card.unwrap())?;
        }

        write!(f, "")
    }
}
