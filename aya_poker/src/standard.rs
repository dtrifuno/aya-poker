use aya_base::{constants::RANK_OFFSET, Hand};

use crate::PokerRankCategory;

include!(concat!(env!("OUT_DIR"), "/holdem.rs"));

/// The strength ranking of a hand in standard poker.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct PokerHandRank(pub u16);

/// Returns the rank of the best standard 5-card poker hand that can be made
/// from the given cards.
///
/// If `hand` contains fewer than 5 cards, the missing cards are considered
/// to be the worst possible kickers for the made hand, i.e. the empty hand
/// ranks as a 6-high, while "Ah As" as pair of aces, with 4, 3 and 2 kickers.
///
/// # Examples
///
/// ```
/// use aya_poker::poker_rank;
///
/// let hand = "3c Js Qd 3h Jc".parse()?;
/// let rank = poker_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
#[inline]
pub fn poker_rank(hand: &Hand) -> PokerHandRank {
    if hand.has_flush() {
        PokerHandRank(HOLDEM_FLUSH_PHF.get(hand.flush_key() as u64))
    } else {
        PokerHandRank(HOLDEM_RANKS_PHF.get(hand.rank_key() as u64))
    }
}

impl PokerHandRank {
    /// Returns the poker hand-ranking category (i.e. high card, pair, etc.)
    /// corresponding to the hand ranking.
    ///
    /// # Examples
    /// ```
    /// use aya_poker::{poker_rank, PokerRankCategory};
    ///
    /// let hand = "Ks Kd Ac 6s 4c Jc Th".parse()?;
    /// let rank = poker_rank(&hand);
    /// assert_eq!(rank.rank_category(), PokerRankCategory::Pair);
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn rank_category(&self) -> PokerRankCategory {
        if self.0 == 0 {
            return PokerRankCategory::Ineligible;
        }

        match self.0 as usize / RANK_OFFSET {
            0 => PokerRankCategory::HighCard,
            1 => PokerRankCategory::Pair,
            2 => PokerRankCategory::TwoPair,
            3 => PokerRankCategory::ThreeOfAKind,
            4 => PokerRankCategory::Straight,
            5 => PokerRankCategory::Flush,
            6 => PokerRankCategory::FullHouse,
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
    #[case::high_card("Js 4c 7h Kd 9c", PokerRankCategory::HighCard)]
    #[case::pair("2d Jc 9c Jd 8h", PokerRankCategory::Pair)]
    #[case::two_pair("Jh 3h Jc 3s 7d 7c 6d", PokerRankCategory::TwoPair)]
    #[case::three_of_a_kind("Th 8c Qs 8h 8d", PokerRankCategory::ThreeOfAKind)]
    #[case::straight("4s 5c 5s 3d 7c 8d 6d", PokerRankCategory::Straight)]
    #[case::flush("Kh 2h 7h 6h Qh 7s 3s", PokerRankCategory::Flush)]
    #[case::full_house("8c Kd 8d 8h 4s Kh 9d", PokerRankCategory::FullHouse)]
    #[case::four_of_a_kind("Ac 9c 5h 5c 7s 5s 5d", PokerRankCategory::FourOfAKind)]
    #[case::straight_flush("2c 8d 9h 7d 4d 5d 6d", PokerRankCategory::StraightFlush)]
    #[case::royal_flush("Ah Kh Jh Th Qh", PokerRankCategory::RoyalFlush)]
    fn rank_category(
        #[case] cards: &str,
        #[case] expected_category: PokerRankCategory,
    ) -> Result<(), ParseError> {
        let hand = cards.parse()?;
        let ranking = poker_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::high_card(&[
        "7s",
        "5h",
        "2d",
        "7s 5c 4h",
        "2c 3c 4d"
    ])]
    #[case::high_card(&[
        "Kc Qs Jd",
        "Kh Qd Jd 2s 3d",
        "Kd Qh Jd 3c",
        "Ks 2c Qd 3s Jd",
        "Kc Qs Jd 2c"
    ])]
    #[case::pair(&[
        "7c 7d",
        "7c 7d 2h",
        "7c 7d 4h",
        "7c 7d 2c 3s 4d",
        "7c 7d 3h 4h",
    ])]
    #[case::straight(&[
        "Ks Th Qs 9c Jh",
        "Kh Td Qd 9c Jc Jd",
        "Kd Tc Qc 9s Jd 9c",
        "Kc 9s Td Qh Jc 7d 8d",
        "Jc Kd Td Qc 9s"
    ])]
    #[case::three_of_a_kind(&[
        "Ac As Ad 5c 6h",
        "Ah As Ac 5c 6s 2c 3c",
        "Ad Ah Ac 5c 6d 3d 4h",
        "Ah As Ac 5c 6d",
        "6h Ad 5c Ah Ac"
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = poker_rank(&h1?);
            let r2 = poker_rank(&h2?);

            assert_eq!(
                r1, r2,
                "{:?} was ranked {:?}, but {:?} is ranked {:?}.",
                &h1, r1, &h2, r2
            );
        }

        Ok(())
    }

    #[test]
    fn rank_ordering() -> Result<(), ParseError> {
        let hands = [
            // High cards
            "",                     // 7 5 4 3 2
            "2d 3s 5d 6d 7h",       // 7 6 5 3 2
            "8h",                   // 8 5 4 3 2
            "2h 3c 8c 6d 7s",       // 8 7 6 3 2
            "6c 4h 3s 2d 7c 8c 9d", // 9 8 7 6 4
            "Th 5c",                // T 5 4 3 2
            "Th 9h 7c 5h 4d",       // T 9 7 5 4
            "Jc",                   // J 5 4 3 2
            "Jc 2s 3d 4c 6s",       // J 6 4 3 2
            "Jd 5s 3h 2s 8h",       // J 8 5 3 2
            "Jh Tc",                // J T 4 3 2
            "Qc",                   // Q 5 4 3 2
            "Qs 6c 7s",             // Q 7 6 3 2
            "Qh 9s",                // Q 9 4 3 2
            "Qd Td 9h",             // Q T 9 3 2
            "Qc 3d Js 7s 8h 9c 5d", // Q J 9 8 7
            "Kh 2h 3s 4s 5c",       // K 5 4 3 2
            "Ad",                   // A 6 4 3 2
            "Ac 6h 5d 4s 2h",       // A 6 5 4 2
            // Pairs
            "3c 3d",             // 3 5 4 2
            "3h 3s 7d 6s 5c",    // 3 7 6 5
            "3h 3s 2d 5c 8c 4h", // 3 8 5 4
            "3h 3s Ad Kd Qd",    // 3 A K Q
            "5s 5d",             // 5 4 3 2
            "5h 5s 7h 8s 9s",    // 5 9 8 7
            "5d 5c Ks 2h 3h",    // 5 K 3 2
            "6s 6d",             // 6 4 3 2
            "6h 6d 5c",          // 6 5 3 2
            "Jc Js Kd Qd Tc",    // J K Q T
            "Jd Jh Ah 2c 3s 5s", // J A 5 3
            "Qc Qs",             // Q 4 3 2
            "Qd Qh 6c",          // Q 6 3 2
            "Qc Qs 6d 4h",       // Q 6 4 2
            "Qs 6s 5h 2h Qh",    // Q 6 5 2
            "Kc Ks 7d",          // K 7 3 2
            "Kc Ks 7c 4s",       // K 7 4 2
            "Ad Ah",             // A 4 3 2
            "Ah Ac 5c 4c 3c",    // A 5 4 3
            // Two pairs
            "2h 2c 3s 3d",          // 3 2 4
            "4c 4s 3d 3h 2s 2h Kh", // 4 3 K
            "4d 4s 3h 3s Ah",       // 4 3 A
            "6h 6s 3d 3c 5c",       // 6 3 5
            "9d 8d 8c 9h 3h Ks",    // 9 8 K
            "Ad 3d Jh Js 3c 8h",    // J 3 A
            "Qs 5s 5h 8h Qh",       // Q 5 8
            "As Ad Qh Qd",          // A Q 2
            "As Ac Qc Qd 5h",       // A Q 5
            // Sets
            "2h 2c 2s",             // 2 4 3
            "2c 9c 2s 2d Ah Jd",    // 2 A J
            "5d 5s 5c 3h 2d",       // 5 3 2
            "5h 5c 5s 7c",          // 5 7 2
            "6h 6c 6s 7h 8h 9h",    // 6 9 8
            "6d 6s 6c 5s Tc",       // 6 T 5
            "9c 9s 9h",             // 9 3 2
            "9h 9d 6h 9s 3d 4h 5c", // 9 6 5
            "Ah As Ad",             // A 3 2
            "Ac Ad 2h As 4c",       // A 4 2
            // Straights
            "Ah 2c 4d 3s 9h 5c Td", // 5
            "2d 4s 3d 6h 5c",       // 6
            "5h 4c 6d 7d Td 8s",    // 8
            "Td 6s 9h Ad Ks 8s 7d", // T
            "Qs Jd Th 9c 8s",       // Q
            "Ah Js Td Qh Jd Kc 8d", // A
            // Flushes
            "7h 5h 4h 3h 2h",       // 7 5 4 3 2
            "4c 2c 7c 3c Ad 6c",    // 7 6 4 3 2
            "3s 4s 5s 8s Js Jd Jc", // J 8 5 4 3
            "4c 7h Jh 9h Qh 5d 8h", // Q J 9 8 7
            "7d Jd Ad 5d 8d",       // A J 8 7 5
            "Ah Kh Qh 9h 8h",       // A K Q 9 8
            // Full houses
            "2c 2s 2d 3h 3c 3d Jh", // 3 2
            "3d 3s 3h 4h 4s",       // 3 4
            "8d 8h Ah Ad 8s",       // 8 A
            "9c 9s 9d Kc Kd",       // 9 K
            "Td Ts Qs Jh Jd Tc 4d", // T J
            "Ts Td Th Qs Qc",       // T Q
            "Jd Js Qs 2h 2d Jc",    // J 2
            "Ac As Kd Kh Ks",       // K A
            "Kd Ks Kc Ah Ac As Qh", // A K
            // Quads
            "2c 2s 2h 2d",          // 2 3
            "2c 2d 2s 2h 4h",       // 2 4
            "2c 2d 2s 2h 3c Ah",    // 2 A
            "4c 4s 4d 4h",          // 4 2
            "4s 4d 4c 4h 3c",       // 4 3
            "4d 4h 4s 4c 2h Ac",    // 4 A
            "6d 6h 6s 6c",          // 6 2
            "6c 6s 6d 6h 3c",       // 6 3
            "6s 6c 6d 6h Qd",       // 6 Q
            "6d 6h 6s 6c 5h Ac Kd", // 6 A
            "8c 8d 8s 8h",          // 8 2
            "8h 8s 8d 8c 5c",       // 8 5
            "Kc Kh Ks Kd Ad",       // K A
            "Ac Ad As Ah",          // A 2
            "Ac Ad As Ah 5s",       // A 5
            "Ac Ad As Ah 4c Ks",    // A K
            // Straight flushes
            "2h 3h Ah 4h 5h",       // 5
            "4c 2c 5c 3c Ad 6c",    // 6
            "2h 3h 4h 5h 6h 7h Kd", // 7
            "8c 5c 6c 7c 9c",       // 9
            "Qd Tc 9d Jd Td Kd",    // K
            // Royal flush
            "Kh Qc Qh Th Jh Ah Ts", // A
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = poker_rank(&h1?);
            let r2 = poker_rank(&h2?);

            assert!(
                r1 < r2,
                "{:?} is ranked {:?}, which is larger than {:?} ({:?}).",
                &h1,
                r1,
                &h2,
                r2
            );
        }

        Ok(())
    }
}
