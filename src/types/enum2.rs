enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}


enum PokerCard1 {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}

fn main() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);

    let c3 = PokerCard1::Spades(5);
    let c4 = PokerCard1::Diamonds('A');
}