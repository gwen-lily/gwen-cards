use std::cmp::Ordering;

use gwen_cards::{card::Card, deck::new_std_deck};
use rand::{seq::SliceRandom, thread_rng, rngs::ThreadRng};


pub fn main() {

    let mut player_1: Player = Player::new(new_std_deck(true));
    let mut player_2: Player = Player::new(new_std_deck(true));

    println!("{:}", "starting war...");
    
    let mut count: u32 = 0;

    while !player_1.deck.is_empty() && !player_2.deck.is_empty() {
        count += 1;
        println!("{:}{:}", "round", count);
        battle(&mut player_1, &mut player_2);
    }

    if player_1.deck.is_empty() {
        println!("{:}", "winner: player 1");
    } else {
        println!("{:}", "winner: player 2");
    }


    
}

struct Player {
    deck: Vec<Card>,
    discard: Vec<Card>,
    contested: Vec<Card>
}

impl Player {
    fn new(deck: Vec<Card>) -> Player {
        Player { deck, discard: vec![], contested: vec![] }
    }
}


fn duel(left: &Card, right: &Card) -> Ordering {
    if left.rank < right.rank {
        Ordering::Less
    } else if left.rank == right.rank {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn battle(left: &mut Player, right: &mut Player) {

    // run loop until at least one player is out of cards
    loop {
        match (left.deck.pop(), right.deck.pop()) {
            (Some(left_card), Some(right_card)) => {
                match duel(&left_card, &right_card) {
                    Ordering::Less => {
                        right.discard.push(left_card);
                        right.discard.push(right_card);

                        if !right.contested.is_empty() {
                            right.discard.append(&mut right.contested);
                            right.discard.append(&mut left.contested);
                        }
                    },
                    Ordering::Greater => {
                        left.discard.push(left_card);
                        left.discard.push(right_card);

                        if !left.contested.is_empty() {
                            left.discard.append(&mut right.contested);
                            left.discard.append(&mut left.contested);
                        }
                    },
                    Ordering::Equal => {
                        left.contested.push(left_card);
                        right.contested.push(right_card);
                    }
                }
            },
            (Some(left_card), None) => {
                // left wins by default
                left.discard.push(left_card);
                left.discard.append(&mut left.contested);
                left.discard.append(&mut right.contested);
                break
            },
            (None, Some(right_card)) => {
                // right wins by default
                right.discard.push(right_card);
                right.discard.append(&mut left.contested);
                right.discard.append(&mut right.contested);
                break
            },
            (None, None) => {
                // If a duel results in a draw with no cards remaining, return each player's contested cards
                right.discard.append(&mut right.contested);
                left.discard.append(&mut left.contested);
                break
            }
        }
    }
    
    // no contests should remain
    assert!(left.contested.is_empty());
    assert!(right.contested.is_empty());

    // seed, condense, shuffle
    let mut rng: ThreadRng = thread_rng();

    left.deck.append(&mut left.discard);
    left.deck.shuffle(&mut rng);

    right.deck.append(&mut right.discard);
    right.deck.shuffle(&mut rng);

}