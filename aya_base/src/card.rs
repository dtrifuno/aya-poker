use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

#[cfg(feature = "colored")]
use colored::{Color, Colorize};

use crate::constants::{CARDS, CARDS_DEBUG_STR, CARDS_STR};
use crate::rank::Rank;
use crate::suit::Suit;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
/// A card from a standard 52-card deck.
pub struct Card {
    pub(crate) key: u64,
    pub(crate) mask: u64,
}

impl Card {
    /// Creates a new card of the given `rank` and `suit`.
    pub fn new(rank: Rank, suit: Suit) -> Self {
        CARDS[4 * (rank as usize) + suit as usize]
    }

    /// Returns the position of the card in a standard 52-card deck ordered by
    /// rank and then suit (deuces to aces, clubs to spades).
    pub fn idx(&self) -> usize {
        (4 * (self.mask.trailing_zeros() % 16) + self.mask.trailing_zeros() / 16) as usize
    }

    /// Returns the rank of the card.
    pub fn rank(&self) -> Rank {
        ((self.mask.trailing_zeros() % 16) as u8)
            .try_into()
            .unwrap()
    }

    /// Returns the suit of the card.
    pub fn suit(&self) -> Suit {
        ((self.mask.trailing_zeros() / 16) as u8)
            .try_into()
            .unwrap()
    }

    /// Returns an ordering where `self` is greater if it has greater rank, or
    /// greater suit if the ranks are equal.
    ///
    /// Note that [`Rank::Ace`] is taken to be the lowest rank, unlike in the
    /// standard aces high ranking which [`Card`] uses to implement
    /// [`PartialOrd`]. The suits are ranked in reverse alphabetical order, with
    /// [`Suit::Clubs`] being the lowest ranking suit, and
    /// [`Suit::Spades`] the greatest.
    pub fn aces_low_cmp(&self, other: &Self) -> Ordering {
        match (self.rank(), other.rank()) {
            (Rank::Ace, Rank::Ace) => self.cmp(other),
            (Rank::Ace, _) => Ordering::Less,
            (_, Rank::Ace) => Ordering::Greater,
            (_, _) => self.cmp(other),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.mask.cmp(&other.mask))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// An error which can be returned when parsing a [`Card`] or [`Hand`](crate::Hand).
///
/// This error is used as the error type for all [`FromStr`] implementations in aya_base.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse value")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseError);
        }

        let rank = s[..1].parse::<Rank>()?;
        let suit = s[1..].parse::<Suit>()?;

        Ok(Self::new(rank, suit))
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", CARDS_DEBUG_STR[self.idx()])
    }
}

impl fmt::Display for Card {
    #[cfg(feature = "colored")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = CARDS_STR[self.idx()];
        write!(f, "{}", base_str.color(self.get_color()))
    }

    #[cfg(not(feature = "colored"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base_str = CARDS_STR[self.idx()];
        write!(f, "{}", base_str)
    }
}

impl Card {
    #[cfg(feature = "colored")]
    fn get_color(&self) -> Color {
        if cfg!(feature = "colored-4color") {
            match self.suit() {
                Suit::Clubs => Color::Green,
                Suit::Diamonds => Color::Blue,
                Suit::Hearts => Color::Red,
                Suit::Spades => Color::Black,
            }
        } else {
            match self.suit() {
                Suit::Hearts | Suit::Diamonds => Color::Red,
                Suit::Spades | Suit::Clubs => Color::Black,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cmp::Ordering;
    use rstest::*;

    #[rstest]
    #[case::two_of_clubs("2c", Card::new(Rank::Two, Suit::Clubs))]
    #[case::eight_of_hearts("8h", Card::new(Rank::Eight, Suit::Hearts))]
    #[case::jack_of_diamonds("Jd", Card::new(Rank::Jack, Suit::Diamonds))]
    #[case::king_of_spades("Ks", Card::new(Rank::King, Suit::Spades))]
    #[case::ace_of_diamonds("Ad", Card::new(Rank::Ace, Suit::Diamonds))]
    fn parse(#[case] s: &str, #[case] expected: Card) -> Result<(), ParseError> {
        let card: Card = s.parse()?;
        assert_eq!(card, expected);
        Ok(())
    }

    #[rstest]
    #[case::empty("")]
    #[case::two_cards("2c 5h")]
    #[case::invalid_rank("Yh")]
    #[case::invalid_suit("Kf")]
    fn invalid_parse(#[case] s: &str) {
        let card = s.parse::<Card>();
        assert_eq!(card, Err(ParseError));
    }

    #[rstest]
    #[case::same_rank_1("3c", "3s")]
    #[case::same_rank_2("Jd", "Jh")]
    #[case::same_suit("5c", "6c")]
    #[case::aces_high("Kh", "Ah")]
    fn order(#[case] lower: &str, #[case] higher: &str) -> Result<(), ParseError> {
        let lower_card = lower.parse::<Card>()?;
        let higher_card = higher.parse::<Card>()?;
        let result = lower_card.cmp(&higher_card);
        assert_eq!(result, Ordering::Less);
        Ok(())
    }
}
