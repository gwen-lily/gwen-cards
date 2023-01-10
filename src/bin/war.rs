use gwen_cards::deck::new_std_deck;
use gwen_cards::card::Card;

pub fn main() {
    let mut deck_1: Vec<Card> = new_std_deck(true);
    let mut deck_2: Vec<Card> = new_std_deck(true);

    while deck_1.len() > 0 && deck_2.len() > 0 {
        continue
    }

}