#![cfg_attr(not(any(std, test)), no_std)]

mod card;
pub mod constants;
mod deck;
mod hand;
mod rank;
mod suit;

pub use card::{Card, ParseError};
pub use constants::CARDS;
pub use deck::{Deck, FullDeck, ShortDeck};
pub use hand::Hand;
pub use rank::Rank;
pub use suit::Suit;