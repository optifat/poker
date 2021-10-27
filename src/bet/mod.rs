pub enum BetType{
    Call,
    Raise,
    Check,
    Fold,
}

pub struct Bet{
    bet_type: BetType,
    amount: u32,
}
