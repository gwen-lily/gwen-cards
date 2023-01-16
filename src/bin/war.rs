use gwen_cards::deck::new_std_deck;
use gwen_cards::card::Card;

pub fn main() {
    let mut p1 = Player::new(new_std_deck(true));
    let mut p2: Player = Player::new(new_std_deck(true));

    while p1.deck.len() > 0 && p2.deck.len() > 0 {
        continue
    }

}


struct Player {
    deck: Vec<Card>,
    discard: Vec<Card>,
}

impl Player {
    fn new(deck: Vec<Card>) -> Player {
        let discard: Vec<Card> = vec![];
        Player { deck, discard }
    }
}

fn duel<'a>(left: &'a mut Player, right: &'a mut Player) -> Option<&'a mut Player> {
    match (left.deck.pop(), right.deck.pop()) {
        // two cards go head to head
        (Some(left_card), Some(right_card)) => {
            if left_card.rank > right_card.rank {
                let winner = left;
                winner.discard.push(left_card);
                winner.discard.push(right_card);
                Some(winner)
            } else if left_card.rank < right_card.rank {
                let winner = right;
                winner.discard.push(left_card);
                winner.discard.push(right_card);
                Some(winner)

            } else {
                // if two cards have equal rank, the result of this duel is the same as the next duel
                if let Some(winner) = duel(left, right) {
                    // if the next duel has a winner, append this duel's cards to that winner
                    winner.discard.push(left_card);
                    winner.discard.push(right_card);
                    Some(winner)
                } else {
                    // if the next duel has no winner, return the cards to each player's discard pile
                    left.discard.push(left_card);
                    right.discard.push(right_card);
                    None
                }
            }
        },
        (Some(left_card), None) => {
            left.discard.push(left_card);
            Some(left)
        },
        (None, Some(right_card)) => {
            right.discard.push(right_card);
            Some(right)
        },
        (None, None) => {
            None
        }

    }
}