enum BetType{
    Call,
    Raise,
    Check,
    Fold,
}

struct Bet{
    bet_type: BetType,
    amount: u32,
}
