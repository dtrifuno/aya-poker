use aya_base::{constants::RANK_OFFSET, Hand, CARDS};

use crate::{insert_cards, PokerRankCategory};

include!(concat!(env!("OUT_DIR"), "/deuce_seven.rs"));

const WORST_2_7_EIGHT_HIGH: u16 = 38124;

/// The strength ranking of a hand in deuce-seven lowball poker.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct DeuceSevenHandRank(pub u16);

/// Returns the rank of the best 5-card deuce-seven lowball poker hand that
/// can be made from the given cards.
///
/// If `hand` contains fewer than 5 cards, the missing cards are considered
/// to be the worst possible kickers for the made hand, i.e. the empty hand
/// ranks as an A-high, while "Ah As" as pair of aces, with K, Q and J kickers.
///
/// # Examples
///
/// ```
/// use aya_poker::deuce_seven_rank;
///
/// let hand = "Kc 4s Ks Tc 2h 9s 8d".parse()?;
/// let rank = deuce_seven_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
#[inline]
pub fn deuce_seven_rank(hand: &Hand) -> DeuceSevenHandRank {
    let mut rank = 0;

    if hand.flush_count() < 5 {
        // Can find the rank directly if the hand cannot make a flush.
        rank = DEUCE_SEVEN_RANKS_PHF.get(hand.rank_key() as u64);
    } else if hand.flush_count() == hand.len() {
        // Or if making a flush is unavoidable.
        rank = DEUCE_SEVEN_FLUSH_PHF.get(hand.flush_key() as u64);
    } else {
        // Otherwise, we iterate over all possible 5 card non-flush hands.
        let mut buffer = [CARDS[0]; 7];
        let cards = insert_cards(hand, &mut buffer);

        const K: usize = 5;
        let mut c = [0; K + 2];

        (0..K).for_each(|i| c[i] = i);
        c[K] = cards.len();
        c[K + 1] = 0;

        let mut j = 1;
        while j <= K {
            let subhand = (0..K).map(|i| cards[c[i]]).collect::<Hand>();
            if !subhand.has_flush() {
                rank = rank.max(DEUCE_SEVEN_RANKS_PHF.get(subhand.rank_key() as u64));
            }

            j = 1;
            while c[j - 1] + 1 == c[j] {
                c[j - 1] = j - 1;
                j += 1;
            }

            c[j - 1] += 1;
        }
    }
    DeuceSevenHandRank(rank)
}

impl DeuceSevenHandRank {
    /// Convert into an 8-or-Better ranking, i.e. make ineligibile if hand is
    /// worse than an 8-high.
    ///
    /// Examples
    ///
    /// ```
    /// use aya_poker::{deuce_seven_rank, DeuceSevenHandRank};
    ///
    /// let ineligible = deuce_seven_rank(&"6h 6c 5c 4s 3d".parse()?);
    /// assert_eq!(ineligible.to_lo_8_rank(), DeuceSevenHandRank(0));
    /// let eight_high = deuce_seven_rank(&"8c 6s 7d 2h 3c 8s".parse()?);
    /// assert_eq!(eight_high, eight_high.to_lo_8_rank());
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn to_lo_8_rank(self) -> DeuceSevenHandRank {
        if self.0 >= WORST_2_7_EIGHT_HIGH {
            self
        } else {
            DeuceSevenHandRank(0)
        }
    }

    /// Returns the poker hand rank category that corresponds to the given
    /// hand rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use aya_poker::{deuce_seven_rank, PokerRankCategory};
    ///
    /// let pair = deuce_seven_rank(&"9s 7c 8s 5d 6c".parse()?);
    /// assert_eq!(pair.rank_category(), PokerRankCategory::Straight);
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn rank_category(&self) -> PokerRankCategory {
        if self.0 == 0 {
            return PokerRankCategory::Ineligible;
        }

        match self.0 as usize / RANK_OFFSET {
            0 => PokerRankCategory::RoyalFlush,
            1 => PokerRankCategory::StraightFlush,
            2 => PokerRankCategory::FourOfAKind,
            3 => PokerRankCategory::FullHouse,
            4 => PokerRankCategory::Flush,
            5 => PokerRankCategory::Straight,
            6 => PokerRankCategory::ThreeOfAKind,
            7 => PokerRankCategory::TwoPair,
            8 => PokerRankCategory::Pair,
            9 => PokerRankCategory::HighCard,
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
    #[case::royal_flush("Ac Qc Kc Tc Jc", PokerRankCategory::RoyalFlush)]
    #[case::straight_flush("7c 5c 6c 3c 4c", PokerRankCategory::StraightFlush)]
    #[case::four_of_a_kind("3c 3s 3d 3h 5c", PokerRankCategory::FourOfAKind)]
    #[case::full_house("Jd Kh Js Jc Kc Kd", PokerRankCategory::FullHouse)]
    #[case::flush("Tc Kc Ac Qc Jc 9c", PokerRankCategory::Flush)]
    #[case::straight("Ah 3c 5s 4d 2h", PokerRankCategory::Straight)]
    #[case::three_of_a_kind("Td 7d 9c 7h 7s", PokerRankCategory::ThreeOfAKind)]
    #[case::two_pair("4h 4s Tc 7s Ts 7h 4c", PokerRankCategory::TwoPair)]
    #[case::pair("Qh Kh 9h 4h Ks", PokerRankCategory::Pair)]
    #[case::high_card("9c 6h Ac 8d 6d Tc Kd", PokerRankCategory::HighCard)]
    fn rank_category(
        #[case] cards: &str,
        #[case] expected_category: PokerRankCategory,
    ) -> Result<(), ParseError> {
        let hand = cards.parse()?;
        let ranking = deuce_seven_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::three_of_a_kind(&[
        "Jc Js Jd",
        "Js Ks Jh Jc",
        "Jd Js Ac Jh",
        "Kc As Jh Js Jd",
        "Jh Js Jd Ah Kh Jc",
    ])]
    #[case::pair(&[
        "Qs Qc",
        "Ah Qs Qc",
        "Qs Kd Qc",
        "Ah Qs Kd Qc Js",
        "Qs Jh Qc",
    ])]
    #[case::high_card(&[
        "Qc Jc Tc Kc",
        "Qc Jc Tc Kc 8h",
        "Qc Jc Tc Kc Ac 8d",
        "Qs Jd Th Kc 8h",
        "Jc Tc Qs Kh 8c",
    ])]
    #[case::high_card(&[
        "7c 6c 5c",
        "7d Ac 6h 5d",
        "Ks 7s 6s Ad 5c",
        "7c 6d As 5c",
        "7c 6s 5d Ah Ks Ad",
    ])]
    #[case::high_card(&[
        "5c",
        "Ks Qh 5c",
        "Qd Ks 5c Jh",
        "Jh 5d",
        "5d Kc Ad"
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = deuce_seven_rank(&h1?);
            let r2 = deuce_seven_rank(&h2?);

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
            // Royal flush
            "Ac Kc Jc Qc Tc", // A
            // Straight flushes
            "Td 9d Jd Kd Qd", // K
            "7d Td Jd 9d 8d", // J
            "6h Th 8h 7h 9h", // T
            "8d 9d 5d 6d 7d", // 9
            "Ah 5h 2h 4h 3h", // 5
            // Quads
            "Ad 8s As Ac Ah", // A 8
            "5d 5s 5c 5h",    // 5 A
            "5h 5c 5s 5d 4d", // 5 4
            // Full houses
            "Kh Jc Kc Ks Js", // K J
            "Tc Ah Ad Td Th", // T A
            "4d 4s 7h 7c 4h", // 4 7
            // Flushes
            "Jd Td Kd 4d Ad", // A K J T 4
            "4c Qc 7c Ac 8c", // A Q 8 7 4
            "4h 7h Kh 6h Qh", // K Q 7 6 4
            "4c Qc 9c Jc 2c", // Q J 9 4 2
            "6s 7s 5s Ts Qs", // Q T 7 5 6
            "9c 5c 3c Tc 2c", // T 9 5 3 2
            "4c 9c 8c 6c 2c", // 9 8 6 4 2
            // Straights
            "Jd Td Kc Ah Qc", // A
            "Td Qc 9c Jd Kh", // K
            "9c 8s Js 7s Th", // Q
            "8c Ts 9h 6h 7c", // T
            "3c 4s 5s 6h 2c", // 6
            "4h 2s 5c Ac 3s", // 5
            // Sets
            "Ac As Ad",       // A K Q
            "As 3d Ks Ah Ac", // A K 3
            "Ad As Ah 5c Td", // A T 5
            "Ad 8d As 3s Ah", // A 8 3
            "Ad Kd Ks Kh 4s", // K A 4
            "Kc Ks Kh Td 7s", // K T 7
            "Kd 6c Kc Kh 8s", // K 8 6
            "Kh 4c 8s Kc Ks", // K 8 4
            "Qs Qh Qd",       // Q A K
            "Th Qh Kc Qs Qd", // Q K T
            "Qc Qh 8d Qd 6c", // Q 8 6
            "7d Qh 4h Qd Qc", // Q 7 4
            // Two pairs
            "4s Ah Th Ac 4d Ad", // A 4 T
            "Th Ks 3h Kh Td",    // K T 3
            "7h 7s Qd Qh Ah",    // Q 7 A
            "9s 7c Qc 7h Qd",    // Q 7 9
            "Qc Qh 6d 6s Ac Ad", // Q 6 A
            "Qd 5d 5h Ks Qh",    // Q 5 K
            "Jh Jc 4h 4d 4s 8c", // J 4 8
            "3s 3h Jd Ac Jc",    // J 3 A
            "3c Ts Th 9d 3s",    // T 3 9
            "2h 7d 9s 2d 9h 2c", // 9 2 7
            "7d 8c 4h 7c 8d",    // 8 7 4
            "4h 8s 8c 5d 5s 5h", // 8 5 4
            "6h 6d 2c 7d 7s",    // 7 6 2
            "Th 5h 3c 3s 5s",    // 5 3 T
            "2d 2c 5d 9d 5s",    // 5 2 9
            // Pairs
            "Ac As",                // A K Q J
            "Ah Ts Ad Qd 9c",       // A Q T 9
            "Kd Ks",                // K A Q J
            "Qh Ks Jd 4d Qd Kd",    // Q K J 4
            "9c Jc Js Kc 6d",       // J K 9 6
            "8c 6s Tc 8d Jd",       // 8 J T 6
            "7c 5h Ks Ts 7s 7h",    // 7 K T 5
            "6h 6d",                // 6 A K Q
            "4d 6d 8s 8h Th 6h Td", // 6 T 8 4
            "8s 5h 6c 6h 2s",       // 6 8 5 2
            "4c Ah 5s 4d 5c Jd",    // 4 J 5 A
            "4d Kc 5c 4h 2h",       // 4 K 5 2
            "2c 2d",                // 2 A K Q
            "2h 5s 2c 9h 8c",       // 2 9 8 5
            "5h 2h 7c 8c 2d",       // 2 8 7 5
            // High cards
            "",                     // A K Q J 9
            "Qh 7h 8h Kh Kc Qd As", // A K Q 8 7
            "Jh As Kd 6d Td",       // A K J T 6
            "2c 9h Ac Kc 7h",       // A K 9 7 2
            "Th Ts Td Qs 9s Js As", // A Q J T 9 [DO NOT REMOVE]
            "4c Ah 2c Td 8d",       // A T 8 4 2
            "4c 2s 3h 6d Ac",       // A 6 4 3 2
            "3d 4d 4h Qs Kd 8c",    // K Q 8 4 3
            "Jc Kd 9h 8d 8h 2s Ac", // K J 9 8 2
            "5c 6d Ks 2s 8d 5s",    // K 8 6 5 2
            "5h Ac Kh 2s 7d 6c Ks", // K 7 6 5 2
            "9s Qs Js Tc Kh 5s",    // Q J T 9 5
            "Jc Qd Qc Tc Ks 3d 4h", // Q J T 4 3
            "Jh 3h 5c 8h As Qd",    // Q J 8 5 3
            "Qc Th Tc 6d 7h 7d 8s", // Q T 8 7 6
            "3h 4h 8c 9s Qd",       // Q 9 8 4 3
            "5c 9c Qc Qh 6h 4s",    // Q 9 6 5 4
            "9d 2h 6c Qh Kh 3c Qs", // Q 9 6 3 2
            "2c 7s 8c Qs 3c",       // Q 8 7 3 2
            "Qh 5h 8s 3h 4s 4c Ad", // Q 8 5 4 3
            "6d Jd 9d Tc 8c",       // J T 9 8 6
            "6d 9h 4d Qh Jh 9d Td", // J T 9 6 4
            "4c 8s Th Jd 4s 5d",    // J T 8 5 4
            "Tc 4c Qd 8s Kc Jd 3d", // J T 8 4 3
            "6h 8s 8d 3d 3s 9d Tc", // T 9 8 6 3
            "3h 8s 4s Qc 5s Tc",    // T 8 5 4 3
            "8c 3s 3c 9h Js 6s 7h", // 9 8 7 6 3
            "2h 6c 7h 5s 2c 9h 7c", // 9 7 6 5 2
            "9h 2s 5h 6s Qh 4d",    // 9 6 5 4 2
            "Jc 6h 4h As 2h 9c 3c", // 9 6 4 3 A
            "3s 7h Qc 8c 5s 3c 4d", // 8 7 5 4 3 [DO NOT REMOVE]
            "Th 5h 4c 2h 7h 8c",    // 8 7 5 4 2
            "4c 5d 6d Tc 3c 7h 2h", // 7 5 4 2 3
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = deuce_seven_rank(&h1?);
            let r2 = deuce_seven_rank(&h2?);

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
