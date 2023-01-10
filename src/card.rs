use strum_macros::EnumIter;

pub type Card = (Rank, Suit);

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, EnumIter, Eq, PartialEq, Clone, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    Red,
    Black,
}