use aya_base::{constants::RANK_OFFSET, Hand};

use crate::PokerRankCategory;

include!(concat!(env!("OUT_DIR"), "/short_deck.rs"));

/// The strength ranking of a hand in six-plus (short-deck) poker.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct ShortDeckHandRank(pub u16);

/// Returns the rank of the best 5-card six-or-better poker hand that can be
/// made from the given cards. 
/// 
/// The caller is responsible for verifying that the hand does not contains
/// any cards of rank less than 6. Otherwise, it silently returns an arbitrary
/// value.
///
/// Note that this is different from calling [`poker_rank`](crate::poker_rank)
/// on a six-or-better hand: in short-deck poker the hand A-9-8-7-6 makes a
/// straight, and flushes rank higher than full houses.
///
/// If `hand` contains fewer than 5 cards, the missing cards are considered
/// to be the worst possible kickers for the made hand, i.e. the empty hand
/// ranks as a J-high, while "Ah As" as a pair of aces, with an 8, 7,
/// and 6 as kickers.
///
/// # Examples
///
/// ```
/// use aya_poker::short_deck_rank;
///
/// let hand = "Ah Ac Jh 8s 6d".parse()?;
/// let rank = short_deck_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
#[inline]
pub fn short_deck_rank(hand: &Hand) -> ShortDeckHandRank {
    if hand.has_flush() {
        ShortDeckHandRank(SIX_PLUS_FLUSH_PHF.get(hand.flush_key() as u64))
    } else {
        ShortDeckHandRank(SIX_PLUS_RANKS_PHF.get(hand.rank_key() as u64))
    }
}

impl ShortDeckHandRank {
    /// Returns the poker hand-ranking category (i.e. high card, pair, etc.)
    /// corresponding to the hand ranking.
    ///
    /// # Examples
    /// ```
    /// use aya_poker::{short_deck_rank, PokerRankCategory};
    ///
    /// let hand = "Qc Qh 7c 8s Jd".parse()?;
    /// let rank = short_deck_rank(&hand);
    /// assert_eq!(rank.rank_category(), PokerRankCategory::Pair);
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn rank_category(&self) -> PokerRankCategory {
        match self.0 as usize / RANK_OFFSET {
            0 => PokerRankCategory::HighCard,
            1 => PokerRankCategory::Pair,
            2 => PokerRankCategory::TwoPair,
            3 => PokerRankCategory::ThreeOfAKind,
            4 => PokerRankCategory::Straight,
            5 => PokerRankCategory::FullHouse,
            6 => PokerRankCategory::Flush,
            7 => PokerRankCategory::FourOfAKind,
            8 => PokerRankCategory::StraightFlush,
            9 => PokerRankCategory::RoyalFlush,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::ParseError;
    use rstest::rstest;

    #[rstest]
    #[case::high_card("6d 8s 7h 9s Ks", PokerRankCategory::HighCard)]
    #[case::pair("Jd 7s 7c Ks Tc", PokerRankCategory::Pair)]
    #[case::two_pair("8d 8h 9c Kc Kd", PokerRankCategory::TwoPair)]
    #[case::three_of_a_kind("Ts Th Tc 9h Qh", PokerRankCategory::ThreeOfAKind)]
    #[case::straight("Ac 6s 7d 8h 9h Kc Ks", PokerRankCategory::Straight)]
    #[case::full_house("9h Ks Kd 9c Jh Jc Kh", PokerRankCategory::FullHouse)]
    #[case::flush("6c 8c 9c Kc Qc", PokerRankCategory::Flush)]
    #[case::four_of_a_kind("Jc Jh Js Jd Kc", PokerRankCategory::FourOfAKind)]
    #[case::straight_flush("Ah 6h 7h 8h 9h", PokerRankCategory::StraightFlush)]
    #[case::royal_flush("As Qs Ts Js Ks 8s 9s", PokerRankCategory::RoyalFlush)]
    fn rank_category(
        #[case] cards: &str,
        #[case] expected_category: PokerRankCategory,
    ) -> Result<(), ParseError> {
        let hand = cards.parse()?;
        let ranking = short_deck_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::high_card(&[
        "",
        "6s 9d",
        "Jc 8h 7h",
        "9c 6c",
        "6c 7s 8d 9h Jc"
    ])]
    #[case::high_card(&[
        "9c 7h",
        "7h",
        "8h",
        "6h",
        "Jc"
    ])]
    #[case::pair(&[
        "6h 6c",
        "6s 6c 8s 7s 9c",
        "6d 6c 8d",
        "6c 6s 9d 7h",
        "6s 6d 7c"
    ])]
    #[case::two_pair(&[
        "7c 7h 6s 6d",
        "7d 7s 6s 6d 8s",
        "7c 7s 6h 6c",
        "7c 7h 6h 6d 8c",
        "7h 8c 7d 6c 6d",
    ])]
    #[case::three_of_a_kind(&[
        "Qc Qd Qh",
        "Qs Qc Qh",
        "Qd Qc Qh 7c 6s",
        "Qh Qc Qs 7h",
        "Qc Qs Qd 6h",
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = short_deck_rank(&h1?);
            let r2 = short_deck_rank(&h2?);

            assert_eq!(
                r1, r2,
                "{:?} was ranked {:?}, but {:?} is ranked {:?}.",
                h1?, r1, h2?, r2
            );
        }

        Ok(())
    }

    #[test]
    fn rank_ordering() -> Result<(), ParseError> {
        let hands = [
            // High card
            "8h",                   // J 9 8 7 6
            "Tc",                   // J T 8 7 6
            "8c 9c 6d Qc 7h",       // Q 9 8 7 6
            "8c Qh 6s Td 7d",       // Q T 8 7 6
            "Jh Kd 7s Th 8h",       // K J T 8 7
            "7h Kc Td Qs 8h",       // K Q T 8 7
            "7h 9c 6d Ks Qd Td",    // Q T 9 7 6
            "Jh 6h Kc Ts Qd",       // K Q J T 6
            "Qh Ks 7d 6d Jh Ts",    // K Q J T 7
            "Kh 8h Qh Tc Jh",       // K Q J T 8
            "Ac Td Jc 9s 6h",       // A J T 9 6
            "8d Qd Ac 6c 7h",       // A Q 8 7 6
            "8h Ah 6s Jh Tc Kh",    // A K J T 8
            "7c As Jd 6c Qd Kd",    // A K Q J 7
            "Qc 9h 8c Kh Jd 6h Ad", // Q J 9 8 6
            // Pair
            "6h 6d",                // 6 9 8 7
            "6c Jh 6d 7h 9s",       // 6 J 9 7
            "6h 8c 6d 7s Qs",       // 6 Q 8 7
            "9s 6h 7d 6d Js 8c Qs", // 6 Q J 9
            "9h 8c 7s 6s Qh Kd 7c", // 7 K Q 9
            "9s 7d Ts Ah 7c 6c",    // 7 A T 9
            "8c 8s",                // 8 9 7 6
            "Ts 8s As 7c 8h",       // 8 A T 7
            "9h 9c",                // 9 8 7 6
            "Jc 9h 9s 8c 7s Qd",    // 9 Q J 8
            "9s Ac 6c 9c 7c Ts Qh", // 9 A Q T
            "6s Td Tc Qs 7h",       // T Q 7 6
            "Kd 6s Td Qh Ts Ad 9h", // T A K Q
            "Jh Jc",                // J 8 7 6
            "Jh 8c Js Kc 9d",       // J K 9 8
            "Qs Qh",                // Q 8 7 6
            "Qc Qs Tc",             // Q T 7 6
            "Kc Qh Ac Jh 6h Qd",    // Q A K J
            "8h Js 9d 7d Kd Qd Ks", // K Q J 9
            "Qd 6h Js Th Ac Ah 9h", // A Q J T
            "Qh Ac Ad Kd 6c",       // A K Q 6
            "Kh 7s Ah 9d 6s Qh Ac", // A K Q 9
            // Two pair
            "9d 6c 7d 6d 7s",       // 7 6 9
            "9s 7s 6c 6h 8h 7d Qh", // 7 6 Q
            "7s 6h Kc 6s 7h",       // 7 6 K
            "6d 7c As 7h Qc 6c 9c", // 7 6 A
            "6s 8c 8h As 6c Kc",    // 8 6 A
            "6c 9s 6d 9h",          // 9 6 7
            "6c Js 6h 9s Kd 9c",    // 9 6 K
            "9s 8h Ks Js 8c Qs 9c", // 9 8 K
            "6c 7d Jh Js 6d 7s Qh", // J 7 Q
            "6h Jd Jh 9h 8d 8h",    // J 8 9
            "Jh Jc Kc 8c 7d Ac 8s", // J 8 A
            "Tc Qd Td Js Jh",       // J T Q
            "7s 6d 7h Qs 6h Qc",    // Q 6 7
            "7s Qs 7c Td Qh Ts 6s", // Q T 7
            "9s Ad 7s As 9d",       // A 9 7
            "9d Ad 9s As Jd 8h Th", // A 9 J
            "7s Ac Ah Jh Ks Kc 7c", // A K J
            "Kd Qc Ks Ad As Qs Td", // A K Q
            // Three of a kind
            "6c 6s 6d",             // 6 8 7
            "6h 7d 6d Js Th 6s 9h", // 6 J T
            "6c 6d Qc Tc 6h",       // 6 Q T
            "6h 8c Jd 8h 9h 8d",    // 8 J 9
            "9h 9s 9c Ah",          // 9 A 6
            "Ts 6s Ac Kd Tc Td Qd", // T A K
            "Jd Jh Js",             // J 7 6
            "Jc 8h Js Jd",          // J 8 6
            "Kc Ks Kh",             // K 7 6
            "Ks Th Kh Kd Jh",       // K J T
            "8c 9c Ah As Jd Qc Ad", // A Q J
            // Straight
            "Ac 9s 8h 6d 7s 8c",    // 9
            "8c Ad Tc Ah 9d 6d 7h", // T
            "8h Th 6c Qd Ac 9s Js", // Q
            "Kc 8c Ts Qs 7h 9d Jh", // K
            "Ks Tc Qc 8d Ac Js",    // A
            // Full house
            "9h 9d 6s 6h 6d",       // 6 9
            "Ts 6h 7s 6d 6s Tc",    // 6 T
            "8h Kh 6d 6s Ks 6c Ad", // 6 K
            "7h 9s 9d 7c 7d",       // 7 9
            "7c Ac Kc Qs 7d Qc 7h", // 7 Q
            "9c Js 7s Th Td Tc 7c", // T 7
            "Th Tc 8h 8c 9s Td",    // T 8
            "8c Th Js 6c Td Jc Jd", // J T
            "Qd 9h Js Jd Qh Jc",    // J Q
            "Jd Ac Jh Ts Kc Jc Ks", // J K
            "Kc Qh Ts Ks Td Qc Kd", // K T
            "Ah Ks Kh Ad Ac Js 8s", // A K
            // Flush
            "Ac 6d Kd 7d 8d 6h 9d", // K 9 8 7 6
            "7s 9s Qs 6s Ks",       // K Q 9 7 6
            "7s 6s Kc As Qd 8s Ts", // A T 8 7 6
            "8c 9c Ac 7c 9h Tc Td", // A T 9 8 7
            "Ah Js 9c Kh 8h Th 9h", // A K T 9 8
            "Jc 8c Kc Ac 7s 6h 9c", // A K J 9 8
            "Th 8d Qd Kd Kc 9d Ad", // A K Q 9 8
            // Four of a kind
            "6c 6s 6d 6h",          // 6 7
            "Qc 9c Kh 6s 6d 6c 6h", // 6 K
            "9h 6h 6c 6d 6s Qc Ac", // 6 A
            "7c 7d Tc Th Td Ts",    // T 7
            "8h Tc Ks Kc Th Kh Kd", // K T
            // Straight flush
            "As 7s Th 9s 6s 8s",    // 9
            "7h Jh 9s 8h 9h Js Th", // J
            "Jd 8d Ah 9d Qd 9c Td", // Q
            "Js Ks 9s 8s Jh Ts Qs", // K
            // Royal flush
            "Ac Tc Jc Kc Qh Qc 6c", // A
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = short_deck_rank(&h1?);
            let r2 = short_deck_rank(&h2?);

            assert!(
                r1 < r2,
                "{:?} is ranked {:?}, which is larger than {:?} ({:?}).",
                h1?,
                r1,
                h2?,
                r2
            );
        }

        Ok(())
    }
}
