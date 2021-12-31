use crate::utility::cards::Cards;

pub struct Deck {
    pub cards: Vec<Cards>,
}

impl Deck {
    fn new(cards: Vec<Cards>) -> Deck {
        Deck { cards }
    }

    pub fn join(mut alice_deck: Deck, mut bob_deck: Deck) -> Deck {
        let mut new_cards: Vec<Cards> = vec![];
        new_cards.append(&mut alice_deck.cards);
        new_cards.push(Cards::KING);
        new_cards.append(&mut bob_deck.cards);
        Deck::new(new_cards)
    }

    pub fn cyclic_shift(&mut self, shift: usize) {
        self.cards.rotate_left(shift % 5);
    }

    pub fn decode(self)->bool{
        let mut opend_deck = self;
        let queen_position = opend_deck.cards.iter().position(|&x| x == Cards::QUEEN).unwrap();
        opend_deck.cyclic_shift(queen_position);
        opend_deck.cards[1] == Cards::QUEEN || opend_deck.cards[4] == Cards::QUEEN
    }
}