use strum::IntoEnumIterator;

use crate::card::{Card, Suit, Rank};
use rand::{self, seq::SliceRandom, thread_rng};



pub fn new_std_deck(shuffled: bool) -> Vec<Card> {
    let mut new_deck: Vec<Card> = vec![];

    for suit in Suit::iter() {
        for rank in Rank::iter() {
            if rank == Rank::Joker {
                if suit == Suit::Diamonds || suit == Suit::Clubs {
                    continue
                }
            }

            let new_card: Card = (rank, suit);
            new_deck.push(new_card);
        }

    }

    if shuffled == true {
        let mut rng = thread_rng();
        new_deck.shuffle(&mut rng)
    }

    new_deck
}
