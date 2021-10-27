mod player;
mod table;

use crate::card_deck::Deck;

pub fn play(){
    let mut table = table::Table::new(10);
    loop{
        let mut deck = Deck::new();
        deck.shuffle();
        deal_flop(&mut deck, &mut table);
    }
}

fn deal_players_cards(deck: &mut Deck, table: &table::Table){
    for player in table.get_players(){
        if player.is_none(){
            continue;
        }
        player.unwrap().deal_card(deck.pop_card());
    }

    for player in table.get_players(){
        if player.is_none(){
            continue;
        }
        player.unwrap().deal_card(deck.pop_card());
    }
}

fn deal_flop(deck: &mut Deck, table: &mut table::Table) -> Result<(), std::io::Error>{
    for _ in 0..3{
        table.deal_card(deck.pop_card())?;
    }
    Ok(())
}
