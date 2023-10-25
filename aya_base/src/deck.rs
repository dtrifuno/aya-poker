use crate::{constants::CARD_COUNT, Card, Rank, CARDS};

/// A custom collection of playing cards that can be dealt in a random order.
pub struct Deck {
    cards: [Card; 52],
    idx: usize,
    end: usize,
    rng: fastrand::Rng,
}

impl Deck {
    /// Creates a new deck containing the given cards shuffled by a random seed.
    #[cfg(std)]
    pub fn new<'a>(cards: impl IntoIterator<Item = &'a Card>) -> Deck {
        let seed = fastrand::u64(..);
        Deck::with_seed(cards, seed)
    }

    /// Creates a new deck containing the given cards shuffled according to the
    /// initial seed.
    pub fn with_seed<'a>(cards: impl IntoIterator<Item = &'a Card>, seed: u64) -> Deck {
        let mut buffer = CARDS;
        let mut count = 0;
        for (i, card) in cards.into_iter().enumerate() {
            if count == CARD_COUNT {
                panic!("deck cannot contain more than 52 cards");
            }

            buffer[i] = *card;
            count += 1;
        }

        Deck {
            cards: buffer,
            idx: 0,
            end: count,
            rng: fastrand::Rng::with_seed(seed),
        }
    }

    ///
    pub fn deal(&mut self, num_cards: usize) -> Option<&[Card]> {
        if num_cards > self.cards.len() {
            return None;
        }

        for i in self.idx..(self.idx + num_cards) {
            self.cards.swap(i, self.rng.usize(i..self.end))
        }

        let result = &self.cards[self.idx..(self.idx + num_cards)];
        self.idx += num_cards;
        Some(result)
    }

    /// Returns the number of cards remaining in the deck.
    pub fn len(&self) -> usize {
        self.end - self.idx
    }

    /// Returns `true` if there are no more cards available in the deck.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Replaces the cards that have been dealt out and shuffles the deck.
    pub fn reset(&mut self) {
        self.idx = 0;
    }
}

/// A standard 52-playing cards deck.
pub struct FullDeck(Deck);

impl FullDeck {
    /// Creates a new 52-card deck shuffled by a random seed.
    #[cfg(std)]
    pub fn new() -> FullDeck {
        let deck = Deck::new(CARDS.iter());
        FullDeck(deck)
    }

    /// Creates a new deck containing the standard 52-playing cards shuffled
    /// according to the initial seed.
    pub fn with_seed(seed: u64) -> FullDeck {
        let deck = Deck::with_seed(CARDS.iter(), seed);
        FullDeck(deck)
    }

    pub fn deal(&mut self, num_cards: usize) -> Option<&[Card]> {
        self.0.deal(num_cards)
    }

    /// Returns the number of cards remaining in the deck.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no more cards available in the deck.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Replaces the cards that have been dealt out and shuffles the deck.
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

/// A deck consisting of the 36 six-or-better cards from a standard deck.
pub struct ShortDeck(Deck);

impl ShortDeck {
    /// Creates a new 36 six-or-better-card deck shuffled by a random seed.
    #[cfg(std)]
    pub fn new() -> ShortDeck {
        let six_plus_cards = CARDS.iter().filter(|&c| c.rank() >= Rank::Six);
        let deck = Deck::new(six_plus_cards);
        ShortDeck(deck)
    }

    /// Creates a new deck containing the 36 six-or-better cards shuffled
    /// according to the given initial seed.
    pub fn with_seed(seed: u64) -> ShortDeck {
        let six_plus_cards = CARDS.iter().filter(|&c| c.rank() >= Rank::Six);
        let deck = Deck::with_seed(six_plus_cards, seed);
        ShortDeck(deck)
    }

    pub fn deal(&mut self, num_cards: usize) -> Option<&[Card]> {
        self.0.deal(num_cards)
    }

    /// Returns the number of cards remaining in the deck.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there are no more cards available in the deck.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Replaces the cards that have been dealt out and shuffles the deck.
    pub fn reset(&mut self) {
        self.0.reset();
    }
}
