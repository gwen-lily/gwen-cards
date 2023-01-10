use gwen_cards::card::Card;
use gwen_cards::deck;


pub fn main() {
    let clean_deck: Vec<Card> = deck::new_std_deck(false);

    for card in clean_deck.iter() {
        println!("{:?} {:?}", card.0, card.1)
    }

    let shuffled_deck: Vec<Card> = deck::new_std_deck(true);

    for card in shuffled_deck.iter() {
        println!("{:?} {:?}", card.0, card.1)
    }
}