use std::error::Error;
use thiserror;

use gwen_cards::{card::{Card, Rank}, deck};

/// Represents game states
#[derive(thiserror::Error, Debug)]
pub enum BlackjackError {
    /// Returned when a count is bussin', respectfully, with the count
    #[error("A count buss'd, respectfully. with the count")]
    Bust(u8)
}

fn round(mut deck: Vec<Card>) {
    let mut dealer: Vec<Card> = vec![];
    let mut player: Vec<Card> = vec![];

    for count in 0u8..2u8 {
        dealer.push(draw(&mut deck));
        player.push(draw(&mut deck));
    }


}

fn draw(deck: &mut Vec<Card>) -> Card {
    if let Some(card) = deck.pop() {
        card
    } else {
        deck.append(&mut deck::new_std_deck(true));
        match deck.pop() {
            Some(card) => card,
            None => panic!("Empty draw of fresh deck"),
        }
    }
}

/// Produce a blackjack ready deck :3
fn get_deck(copies: usize) -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    let mut count: usize = 0;

    while count < copies {
        count += 1;
        let mut new_deck: Vec<Card> = deck::new_std_deck(true);
        deck.append(&mut new_deck);
    }

    deck
}



pub fn main() {

}

/// Return valid counts within the range of 2 and 21 from the provided cards 
fn count(cards: Vec<Card>) -> Result<u8, BlackjackError> {
    // blackjack hand size starts at 2
    assert!(cards.len() >= 2);

    let mut raw_count: u8 = cards.iter().map(|c| value(c)).sum();

    // include high ace (if valid)
    for card in cards {
        if card.rank == Rank::Ace && raw_count <= 11 {
            raw_count += 10;
            break; // impossible to produce valid count with two high aces
        }
    }

    // check whether count is valid or bust
    if raw_count <= 21 {
        Ok(raw_count)
    } else {
        Err(BlackjackError::Bust(raw_count))
    }
}


fn value(card: &Card) -> u8 {
    use Rank::*;

    match card.rank {
        Two => 2,
        Three => 3,
        Four => 4,
        Five => 5,
        Six => 6,
        Seven => 7,
        Eight => 8,
        Nine => 9,
        Ten | Jack | Queen | King => 10,
        Ace => 1,
        Joker => panic!("joker in blackjack"),
    }
}