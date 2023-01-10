pub type Card = (Rank, Suit);


pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
    Joker
}

pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
    Default,
}

pub enum Color {
    Red,
    Black,
}