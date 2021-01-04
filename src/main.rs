

enum CardOrder {
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
}

impl CardOrder {
    fn list() -> Vec<CardOrder> {
        vec![
            CardOrder::Two, CardOrder::Three, CardOrder::Four,
            CardOrder::Five, CardOrder::Six, CardOrder::Seven,
            CardOrder::Eight, CardOrder::Nine, CardOrder::Ten,
            CardOrder::Jack, CardOrder::Queen, CardOrder::King,
            CardOrder::Ace,
        ]
    }

    fn order(&self) -> u8 {
        match self {
            // CardOrder::Ace => 1, // Only for a straight.
            CardOrder::Two => 2,
            CardOrder::Three => 3,
            CardOrder::Four => 4,
            CardOrder::Five => 5,
            CardOrder::Six => 6,
            CardOrder::Seven => 7,
            CardOrder::Eight => 8,
            CardOrder::Nine => 9,
            CardOrder::Ten => 10,
            CardOrder::Jack => 11,
            CardOrder::Queen => 12,
            CardOrder::King => 13,
            CardOrder::Ace => 14,
        }
    }

    fn to_short_name(&self) -> String {
        match self {
            CardOrder::Ten => "T".to_string(),
            CardOrder::Jack => "J".to_string(),
            CardOrder::Queen => "Q".to_string(),
            CardOrder::King => "K".to_string(),
            CardOrder::Ace => "A".to_string(),
            x => x.order().to_string()
        }.into()
    }
}

#[derive(Clone)]
enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl CardSuit {
    fn list() -> Vec<CardSuit> {
        vec![
            CardSuit::Spades,
            CardSuit::Hearts,
            CardSuit::Diamonds,
            CardSuit::Clubs
        ]
    }

    fn to_symbol(&self) -> String {
        match self {
            CardSuit::Spades => "♠",
            CardSuit::Hearts => "♥",
            CardSuit::Diamonds => "♦",
            CardSuit::Clubs => "♣"
        }.into()
    }
}

struct PlayingCard {
    value: CardOrder,
    suit: CardSuit,
}

impl PlayingCard {
    fn to_string(&self) -> String {
        format!("{}{}", self.value.to_short_name(), self.suit.to_symbol())
    }
}

struct CardDeck {
    cards: Vec<PlayingCard>
}

impl CardDeck {
    fn new() -> CardDeck {
        let mut deck = CardDeck { cards: Vec::new() };
        CardSuit::list().iter().for_each(|suit| {
            CardOrder::list().into_iter().for_each(|value| {
                let card = PlayingCard { value, suit: suit.clone() };
                deck.cards.push(card)
            });
        });
        deck
    }

    fn shuffle(&mut self) {

    }

    fn draw_card(&mut self) -> PlayingCard {

    }
}

enum Hand {
    Empty, // In between games, sitting out.
    Texas([PlayingCard; 2]),
    Omaha([PlayingCard; 4]),
}


fn main() {
    println!("Welcome to the card deck shuffler");

    let deck = CardDeck::new();
    deck.cards.iter().for_each(|card|{
       println!("Card: {}", card.to_string());
    });

    println!("Program end");
}
