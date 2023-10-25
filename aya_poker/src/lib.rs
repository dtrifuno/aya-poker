//! AyaPoker is a Rust library for fast poker hand evaluation based on the
//! [OMPEval C++ hand evaluator](https://github.com/zekyll/OMPEval), with
//! support for most popular poker variants.
//!
//! # Features
//!
//! - Can be used to rank hands from standard poker, ace-to-five lowball,
//!   deuce-to-seven lowball, six-or-better (short-deck), Omaha, Omaha Hi/Lo,
//!   Badugi or Baduci.
//! - Can evaluate hands with 0 to 7 cards, with the missing cards counting as
//!   the worst possible kickers, allowing for use in stud poker games.
//! - Uses compile-time generated perfect hash function lookup tables for
//!   excellent runtime performance and fast initialization.
//! - Has extensive suite of tests to ensure correct implementation of the hand
//!   ranking rules for each variant.
//!
//! # Flags
//!
//! This crate has the following Cargo features:
//!
//! - `std`: By default, `aya_poker` is a `!#[no_std]` crate, but can be
//!   compiled with the `std` feature in order to allow the initialization of
//!   `Deck`s with system-generated random seeds.
//! - `colored`: Use [`colored`](https://crates.io/crates/colored) to display
//!   cards and hands in color.
//! - `colored-4color`: Same as `colored`, but using a four-color deck.
//!
//! # Example
//!
//! ```
//! use std::cmp::Ordering;
//!
//! use aya_poker::{base::*, deck::Deck, poker_rank};
//!
//! const SEED: u64 = 42;
//! const SAMPLE_SIZE: usize = 100_000;
//!
//! fn main() -> Result<(), ParseError> {
//!     // We can initialize cards by specifying the rank and suit,
//!     // and then use collect to combine the cards into a Hand
//!     let player = [
//!         Card::new(Rank::King, Suit::Hearts),
//!         Card::new(Rank::King, Suit::Clubs),
//!     ]
//!     .iter()
//!     .collect();
//!
//!     // Or, first parse the cards from strings
//!     let opponent = ["Js".parse::<Card>(), "Qs".parse::<Card>()]
//!         .iter()
//!         .copied()
//!         .collect::<Result<Hand, ParseError>>()?;
//!
//!     // Or, parse a space-separated string of cards as a Hand
//!     let board = "Ah 5s Ts".parse()?;
//!
//!     let equity = equity_calculator(&player, &opponent, &board);
//!     println!(
//!         "{} has {:.1}% equity on {} against {}.",
//!         player,
//!         100.0 * equity,
//!         board,
//!         opponent
//!     );
//!
//!     Ok(())
//! }
//!
//! fn equity_calculator(
//!     player: &Hand,
//!     opponent: &Hand,
//!     board: &Hand,
//! ) -> f64 {
//!     // To simulate board run-outs, we begin by preparing a deck
//!     // that doesn't contain the already dealt-out cards
//!     let available_cards = CARDS
//!         .iter()
//!         .filter(|c| !player.contains(c))
//!         .filter(|c| !opponent.contains(c))
//!         .filter(|c| !board.contains(c));
//!     let mut deck = Deck::with_seed(available_cards, SEED);
//!
//!     let mut pots_won = 0.0;
//!     for _ in 0..SAMPLE_SIZE {
//!         // Then, for each run we draw cards to complete the board
//!         deck.reset();
//!         let missing = 5 - board.len();
//!         let complete_board = board
//!             .iter()
//!             .chain(deck.deal(missing).unwrap().iter())
//!             .collect::<Hand>();
//!
//!         // Evaluate the player's hand given the completed board
//!         let mut player_hand = *player;
//!         player_hand.extend(complete_board.iter());
//!         let player_rank = poker_rank(&player_hand);
//!
//!         // Evaluate the opponent's hand
//!         let mut opponent_hand = *opponent;
//!         opponent_hand.extend(complete_board.iter());
//!         let opponent_rank = poker_rank(&opponent_hand);
//!
//!         // And record the player's share of the pot for the run
//!         match player_rank.cmp(&opponent_rank) {
//!             Ordering::Greater => pots_won += 1.0,
//!             Ordering::Less => {}
//!             Ordering::Equal => pots_won += 0.5,
//!         };
//!     }
//!
//!     pots_won / SAMPLE_SIZE as f64
//! }
//! ```

#![cfg_attr(not(any(std, test)), no_std)]

use quickdiv::DivisorU64;

mod ace_five;
mod baduci;
mod badugi;
mod deuce_seven;
mod display;
mod omaha;
mod short_deck;
mod standard;

/// Basic types for playing card games.
pub mod base {
    pub use aya_base::{Card, Hand, ParseError, Rank, Suit, CARDS};
}

/// Deck types optimized for fast shuffling suitable for use in simulators.
pub mod deck {
    pub use aya_base::{Deck, FullDeck, ShortDeck};
}

pub use ace_five::{ace_five_rank, AceFiveHandRank};
pub use baduci::{baduci_rank, BaduciHandRank};
pub use badugi::{badugi_rank, BadugiHandRank};
pub use deuce_seven::{deuce_seven_rank, DeuceSevenHandRank};
pub use omaha::{omaha_lo_rank, omaha_rank};
pub use short_deck::{short_deck_rank, ShortDeckHandRank};
pub use standard::{poker_rank, PokerHandRank};

struct MiniPhf {
    buckets_len: DivisorU64,
    len: DivisorU64,
    values: &'static [u16],
    pilots: &'static [u32],
}

impl MiniPhf {
    pub const fn new(values: &'static [u16], pilots: &'static [u32]) -> MiniPhf {
        let buckets_len = DivisorU64::new(pilots.len() as u64);
        let len = DivisorU64::new(values.len() as u64);
        MiniPhf {
            buckets_len,
            len,
            values,
            pilots,
        }
    }

    #[inline]
    pub fn get(&self, key: u64) -> u16 {
        let pilot = self.pilots[(key % self.buckets_len) as usize] as u64;
        let idx = ((key ^ pilot) % self.len) as usize;
        self.values[idx]
    }
}

/// A poker hand-ranking category, i.e. a straight, a flush, etc.
///
/// Note we do not implement [`PartialOrd`] since we use the same ranking
/// categories for both regular and lowball poker variants.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum PokerRankCategory {
    /// A hand without a valid ranking, for example a 9-high in a 8 or better
    /// lowball game.
    Ineligible,
    /// A valid hand that does not fall into any of the other categories.
    HighCard,
    /// Two cards of one rank, and three cards of three other ranks.
    Pair,
    /// Two cards of one rank, two cards of another rank and a fifth card of
    /// a different, third rank.
    TwoPair,
    /// Three cards of the same rank, and two cards of two other ranks.
    ThreeOfAKind,
    /// Five cards of sequential rank, with at least two different suits.
    Straight,
    /// Five cards of the same suit, but without sequential rank.
    Flush,
    /// Three cards of one rank and two cards of another rank.
    FullHouse,
    /// Four cards of the same rank and one card of another rank.
    FourOfAKind,
    /// Five cards of sequential rank, all of the same suit, excluding an
    /// ace-high sequence.
    StraightFlush,
    /// The sequence A-K-Q-J-T all of the same suit, i.e. an ace-high
    /// straight flush.
    RoyalFlush,
}

impl core::fmt::Display for PokerRankCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PokerRankCategory::Ineligible => write!(f, "Ineligible"),
            PokerRankCategory::HighCard => write!(f, "High Card"),
            PokerRankCategory::Pair => write!(f, "Pair"),
            PokerRankCategory::TwoPair => write!(f, "Two Pair"),
            PokerRankCategory::ThreeOfAKind => write!(f, "Three of a Kind"),
            PokerRankCategory::Straight => write!(f, "Straight"),
            PokerRankCategory::Flush => write!(f, "Flush"),
            PokerRankCategory::FullHouse => write!(f, "Full House"),
            PokerRankCategory::FourOfAKind => write!(f, "Four of a Kind"),
            PokerRankCategory::StraightFlush => write!(f, "Straight Flush"),
            PokerRankCategory::RoyalFlush => write!(f, "Royal Flush"),
        }
    }
}

/// A Badugi/Baduci hand-ranking category corresponding to the size
/// of the made hand.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum BadugiRankCategory {
    /// A single card.
    OneCard,
    /// Two cards with different suits and ranks.
    TwoCards,
    /// Three cards with three distinct ranks and suits.
    ThreeCards,
    /// Four cards with four distinct ranks and suits.
    FourCards,
}

impl core::fmt::Display for BadugiRankCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BadugiRankCategory::OneCard => write!(f, "One Card"),
            BadugiRankCategory::TwoCards => write!(f, "Two Cards"),
            BadugiRankCategory::ThreeCards => write!(f, "Three Cards"),
            BadugiRankCategory::FourCards => write!(f, "Four Cards"),
        }
    }
}

fn insert_cards<'a>(hand: &base::Hand, dest: &'a mut [base::Card]) -> &'a [base::Card] {
    let n = hand.len();
    for (i, &card) in hand.iter().enumerate() {
        dest[i] = card;
    }
    &dest[..n]
}
