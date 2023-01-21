use strum_macros::EnumIter;


#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}


#[derive(EnumIter, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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


impl Card {
    fn color(&self) -> Color {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            _ => Color::Black
        }
    }
}
