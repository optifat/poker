#[derive(Copy, Clone)]
pub struct Card{
    value: u8,
    suit: u8,
}

const CARD_VALUES: [&'static str; 13] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

impl Card{
    pub fn new(value: u8, suit: u8) -> Result<Self, std::io::Error>{
        if value > 12{
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Wrong card value"));
        }

        if suit > 3{
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Wrong card suit"));
        }

        Ok(Card{value, suit})
    }
}

impl std::fmt::Display for Card{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        let card_value = CARD_VALUES[self.value as usize];
        let card_suit = std::char::from_u32(0x2660 + self.suit as u32).unwrap();
        write!(f, "{}{}", card_suit, card_value)
    }
}
