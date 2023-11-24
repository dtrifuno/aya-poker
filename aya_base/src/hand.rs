use core::fmt;
use core::iter::FusedIterator;
use core::str::FromStr;

use crate::card::{Card, ParseError};
use crate::constants::{
    CARD_COUNT, FLUSH_CHECK_MASK32, FLUSH_CHECK_MASK64, MAX_HAND_SIZE, SUITS_SHIFT,
};
use crate::CARDS;

/// An unordered collection of 0-7 cards from a standard 52-card deck.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hand {
    key: u64,
    mask: u64,
}

impl Hand {
    /// Constructs an empty [`Hand`].
    pub fn new() -> Hand {
        Hand {
            key: 0x3333 << SUITS_SHIFT,
            mask: 0,
        }
    }

    /// Returns the number of cards of the most common suit in the hand.
    #[inline]
    pub fn flush_count(&self) -> usize {
        let flush_check_bits = (self.key >> SUITS_SHIFT) as u16;

        let res = (0..4)
            .map(|i| (flush_check_bits >> (4 * i)) & 0xf)
            .max()
            .unwrap() as usize;
        res - 3
    }

    /// Returns `true` if the hand contains the given card.
    pub fn contains(&self, card: &Card) -> bool {
        self.mask & card.mask != 0
    }

    /// Returns the total number of cards in the hand.
    #[inline]
    pub fn len(&self) -> usize {
        (self.counters() & 0b1111) as usize
    }

    /// Returns `true` if the hand does not contain any cards.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the hand contains at least 5 cards of the same suit.
    #[inline]
    pub fn has_flush(&self) -> bool {
        self.key & FLUSH_CHECK_MASK64 != 0
    }

    /// Inserts a card into the hand, but may leave the hand in an invalid
    /// state.
    ///
    /// The caller is responsible for first verifying that:
    /// 1. The card is not already present in the hand, and
    /// 2. The hand does not already contain 7 cards.
    #[inline]
    pub fn insert_unchecked(&mut self, card: &Card) {
        self.key += card.key;
        self.mask |= card.mask;
    }

    /// Returns `true` if hand does not contain any cards of rank less than 6.
    #[inline]
    pub fn is_six_plus(&self) -> bool {
        self.mask & 0x000f000f000f000f == 0
    }

    /// Returns `true` if the two hands have no cards in common.
    #[inline]
    pub fn is_disjoint(&self, other: &Hand) -> bool {
        self.mask & other.mask == 0
    }

    /// Returns a key corresponding uniquely to the ranks (with multiplicity)
    /// present in the hand.
    #[inline]
    pub fn rank_key(&self) -> u32 {
        self.key as u32
    }

    /// Returns a key corresponding uniquely to the ranks of the suit from
    /// which a flush can be made present in the hand.
    ///
    /// If the hand does not contain a flush, it silently returns an arbitrary
    /// value.
    #[inline]
    pub fn flush_key(&self) -> u16 {
        let flush_check_bits = self.counters() & FLUSH_CHECK_MASK32;
        let shift = 48 - 4 * flush_check_bits.leading_zeros();

        (self.mask >> shift) as u16
    }

    /// Returns a key which corresponds to what ranks are present in what
    /// number, and which cards have the same suit, but not the particular
    /// suits.
    ///
    /// You can think of this key as a canonical representative of the hand
    /// in the quotient space under the S_4 group action which permutes suits.
    #[inline]
    pub fn canonical_key(&self) -> u64 {
        let mut arr = [3, 2, 1, 0].map(|i| ((self.mask >> (16 * i)) & 0x1fff) as u16);

        if arr[0] > arr[2] {
            arr.swap(0, 2)
        }

        if arr[1] > arr[3] {
            arr.swap(1, 3)
        }

        if arr[0] > arr[1] {
            arr.swap(0, 1)
        }

        if arr[2] > arr[3] {
            arr.swap(2, 3)
        }

        if arr[1] > arr[2] {
            arr.swap(1, 2)
        }

        ((arr[3] as u64) << 48)
            | ((arr[2] as u64) << 32)
            | ((arr[1] as u64) << 16)
            | (arr[0] as u64)
    }

    /// Returns an iterator over all cards in the hand.
    pub fn iter(&self) -> Iter {
        Iter {
            left_idx: 0,
            right_idx: CARD_COUNT as u8 - 1,
            len: self.len() as u8,
            hand: self,
        }
    }

    fn counters(&self) -> u32 {
        (self.key >> 32) as u32
    }
}

impl Default for Hand {
    fn default() -> Hand {
        Hand::new()
    }
}

impl Extend<Card> for Hand {
    #[inline]
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        for card in iter {
            self.insert_unchecked(&card);
        }
    }
}

impl<'a> Extend<&'a Card> for Hand {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a Card>>(&mut self, iter: T) {
        Hand::extend(self, iter.into_iter().copied())
    }
}

impl FromIterator<Card> for Hand {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut hand = Hand::new();
        hand.extend(iter);
        hand
    }
}

impl<'a> FromIterator<&'a Card> for Hand {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a Card>>(iter: T) -> Self {
        Hand::from_iter(iter.into_iter().copied())
    }
}

impl FromStr for Hand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim() == "" {
            Ok(Hand::new())
        } else {
            let mut hand = Hand::new();
            for result in s.trim().split(' ').map(Card::from_str) {
                let card = result?;
                hand.insert_unchecked(&card);
            }

            if hand.len() as u32 != hand.mask.count_ones()
                || hand.mask.count_ones() > MAX_HAND_SIZE as u32
            {
                Err(ParseError)
            } else {
                Ok(hand)
            }
        }
    }
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_entry = true;
        for card in self.iter() {
            if first_entry {
                write!(f, "{:?}", card)?;
                first_entry = false;
            } else {
                write!(f, " {:?}", card)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_entry = true;
        for card in self.iter() {
            if first_entry {
                write!(f, "{}", card)?;
                first_entry = false;
            } else {
                write!(f, " {}", card)?;
            }
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = &'a Card;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    left_idx: u8,
    right_idx: u8,
    len: u8,
    hand: &'a Hand,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Card;

    fn next(&mut self) -> Option<Self::Item> {
        while self.left_idx <= self.right_idx {
            let card = &CARDS[self.left_idx as usize];
            self.left_idx += 1;

            if self.hand.contains(card) {
                self.len -= 1;
                return Some(card);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while self.left_idx <= self.right_idx {
            let card = &CARDS[self.right_idx as usize];

            if self.hand.contains(card) {
                self.len -= 1;
                return Some(card);
            }
            self.right_idx -= 1;
        }
        None
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}
impl<'a> FusedIterator for Iter<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::duplicate_card(&"Ah 5h 4c 3c Ah")]
    #[case::eight_cards(&"As Ks Qs Js Ts Ac Kc Qc")]
    fn failed_parse(#[case] s: &str) {
        let result = s.parse::<Hand>();
        assert!(result.is_err());
    }

    #[rstest]
    #[case(&["Ah"], 1)]
    #[case(&["Ah", "As"], 2)]
    #[case(&["Jh", "Tc", "7h", "5s"], 4)]
    fn add_cards(#[case] cards: &[&str], #[case] expected_count: usize) -> Result<(), ParseError> {
        let mut hand = Hand::new();

        for &card in cards {
            let card = card.parse()?;
            hand.insert_unchecked(&card);
        }

        assert_eq!(hand.len(), expected_count);
        Ok(())
    }

    #[rstest]
    #[case(&[])]
    #[case(&["4c"])]
    #[case(&["7s", "Jc"])]
    #[case(&["As", "Qc", "Ah", "3h"])]
    fn retrieve_cards(#[case] cards: &[&str]) -> Result<(), ParseError> {
        let mut hand = Hand::new();
        for &card in cards {
            let card = card.parse()?;
            hand.insert_unchecked(&card);
        }

        assert_eq!(hand.len(), cards.len());
        for &card in cards {
            let card = card.parse().unwrap();
            assert!(hand.contains(&card));
        }

        Ok(())
    }
}
