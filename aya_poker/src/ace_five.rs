use aya_base::{constants::RANK_OFFSET, Hand};

use crate::PokerRankCategory;

include!(concat!(env!("OUT_DIR"), "/ace_five.rs"));

/// The numeric value of the ace-five lowball poker ranking of a 8-7-6-5-4 hand.
const WORST_A_5_EIGHT_HIGH: u16 = 21712;

/// The strength ranking of a hand in ace-five lowball poker.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct AceFiveHandRank(pub u16);

/// Returns the rank of the best 5-card ace-five lowball poker hand that
/// can be made from the given cards.
///
/// If `hand` contains fewer than 5 cards, the missing cards are considered
/// to be the worst possible kickers for the made hand, i.e. the empty hand
/// ranks as a K-high, while "Ah As" as pair of aces, with K, Q and J kickers.
///
/// # Examples
///
/// ```
/// use aya_poker::ace_five_rank;
///
/// let hand = "Ah 6c 8s Qd Jd 3h".parse()?;
/// let rank = ace_five_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
#[inline]
pub fn ace_five_rank(hand: &Hand) -> AceFiveHandRank {
    AceFiveHandRank(ACE_FIVE_RANKS_PHF.get(hand.rank_key() as u64))
}

impl AceFiveHandRank {
    /// Converts into an 8-or-better ranking, i.e. returns Ineligibile if hand
    /// is worse ranked than an 8-high.
    ///
    /// # Examples
    ///
    /// ```
    /// use aya_poker::{ace_five_rank, AceFiveHandRank};
    ///
    /// let ineligible = ace_five_rank(&"9h 4h 3d 5d 2c".parse()?);
    /// assert_eq!(ineligible.to_lo_8_rank(), AceFiveHandRank(0));
    /// let six_high = ace_five_rank(&"6s 5s 4s 3s 2s".parse()?);
    /// assert_eq!(six_high, six_high.to_lo_8_rank());
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn to_lo_8_rank(&self) -> AceFiveHandRank {
        if self.0 >= WORST_A_5_EIGHT_HIGH {
            *self
        } else {
            AceFiveHandRank(0)
        }
    }

    /// Returns the poker hand rank category that corresponds to the given
    /// hand rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use aya_poker::{ace_five_rank, PokerRankCategory};
    ///
    /// let pair = ace_five_rank(&"Js 5c 8s 4d Jc".parse()?);
    /// assert_eq!(pair.rank_category(), PokerRankCategory::Pair);
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn rank_category(&self) -> PokerRankCategory {
        if self.0 == 0 {
            return PokerRankCategory::Ineligible;
        }

        match self.0 as usize / RANK_OFFSET {
            0 => PokerRankCategory::FourOfAKind,
            1 => PokerRankCategory::FullHouse,
            2 => PokerRankCategory::ThreeOfAKind,
            3 => PokerRankCategory::TwoPair,
            4 => PokerRankCategory::Pair,
            5 => PokerRankCategory::HighCard,
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
    #[case::four_of_a_kind("Th Ts Td Tc 3c", PokerRankCategory::FourOfAKind)]
    #[case::full_house("8s 8h Ks Kc 8c", PokerRankCategory::FullHouse)]
    #[case::three_of_a_kind("Jh 2c 9d Js Jd", PokerRankCategory::ThreeOfAKind)]
    #[case::two_pair("3s 8d 5s 5d 8s", PokerRankCategory::TwoPair)]
    #[case::pair("5h 5c 3d 9s 9c Kd Ks", PokerRankCategory::Pair)]
    #[case::high_card("8h 6d 3d 8s 7c 5h 8c", PokerRankCategory::HighCard)]
    fn rank_category(
        #[case] cards: &str,
        #[case] expected_category: PokerRankCategory,
    ) -> Result<(), ParseError> {
        let hand = cards.parse()?;
        let ranking = ace_five_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::two_pair(&[
        "9c 9h 7s 7d",
        "9d Kc 9s 7c 7h",
        "9c 9h Kh 7s Kc 7d",
        "9h Kd 9c 7s 7d",
        "9c 9h Kc 7c Ks 7h Kd",
    ])]
    #[case::pair(&[
        "8c 8d",
        "8h 8s Kh",
        "8h 8s Qs",
        "8h 8s Kc Qs Jd",
        "8h 8s Kc Qd Ks Jh Kd",
    ])]
    #[case::high_card(&[
        "Ks Qd Jc Th 9s",
        "Kh Qc Js Td 9c Kc Ks",
        "Jc Td",
        "Qd",
        "9c",
    ])]
    #[case::high_card(&[
        "7c 5h Qc",
        "7s 5d Kh Qh",
        "7s 5c Kc Qd Jh",
        "7h 5c",
        "Ks Jd Qs 7d 5c 5s 5d"
    ])]
    #[case::high_card(&[
        "8d",
        "8h Kc",
        "8d Qd",
        "Kc Qc Js Td 8h",
        "8c Qd Js Kh Tc Ks Qs"
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = ace_five_rank(&h1?);
            let r2 = ace_five_rank(&h2?);

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
            // Quads
            "Kc Ks Kd Kh",    // K Q
            "Kc Kh Ks Kd Jd", // K J
            "Ks Kd Kc Kh 5c", // K 5
            "Qh Qs Qc Qd",    // Q K
            "Qh Qs Qc Qd Td", // Q T
            "Qh Qs Qc Qd Ad", // Q A
            "6c 5c 5h 5s 5d", // 5 6
            "Ac Ah As Ad 2d", // A 2
            // Full houses
            "Kc Kh Kd Ks Qh Qc Qd", // K Q
            "Kh Kd Ks Jc Js Jd Jh", // J K
            "6c 4s 6h 4d 6s",       // 6 4
            "3c 6d 3d 6h 6s",       // 6 3
            "6h 6c 6s 4d 4h 4s",    // 4 6
            "3c 9c 9s 3d 3h",       // 3 9
            // Sets
            "Kh Kc Ks 2c Ad",    // K 2 A
            "8c Jh 8s 2h 8d",    // 8 J 2
            "8c 7h 8h 6h 8s",    // 8 7 6
            "4c 4s 4d",          // 4 K Q
            "3c 3s 3d",          // 3 K Q
            "3c 8h 3s 3d",       // 3 K 8
            "3c 3s 3d 3h 5d 6d", // 3 6 5
            // Two pairs
            "Kc Ks 9h 9d",       // K 9 Q
            "Kc Qs Kd Qh Tc Ts", // Q T K
            "5c 5s 4d 4h",       // 5 4 K
            "Qc 4s 5h 4d 5d",    // 5 4 Q
            "6c 6h 3d 3s Ah Ad", // 3 A 6
            // Pairs
            "Ks Kh",                // K Q J T
            "Kc Ks 9h",             // K Q J 9
            "As Ks 2c Kd Kc 9h",    // K 9 2 A
            "Kh Tc 3s Jh Jd",       // J K T 3
            "Js 3d Jc 5h 4s",       // J 5 4 3
            "7c 7s 6d 4s Kd",       // 7 K 6 4
            "6s 6c",                // 6 K Q J
            "8s Jc 6d 6h Kh",       // 6 K J 8
            "Js Qc 5c 5h 4c",       // 5 Q J 4
            "4s 4d",                // 4 K Q J
            "6c 4c 8c Qc 4d",       // 4 Q 8 6
            "8c 4d 5h Th 4c",       // 4 T 8 5
            "2d 4h 6c 4c Ts",       // 4 T 6 2
            "4c 4s 5s 5d 8c 9h 4d", // 4 9 8 5
            "3s 3h Ac 2c Qh",       // 3 Q 2 A
            // High cards
            "",                     // K Q J T 9
            "Qs Ts 7s",             // K Q J T 7
            "Kh Jh 6s Qh 6d Th",    // K Q J T 6
            "Kc 7h 6d",             // K Q J 7 6
            "2h Jd Qs 5s Ks 5d",    // K Q J 5 2
            "Th 9s 8c",             // K Q T 9 8
            "Kd 8s Qd Kc Ts Kh 6c", // K Q T 8 6
            "9c 7s 6h",             // K Q 9 7 6
            "7s Ad 9h Kh Qd",       // K Q 9 7 A
            "4h Ah Kd Qs 2h",       // K Q 4 2 A
            "3s Js 6s 6h Kd Ac",    // K J 6 3 A
            "9s As Kd Td Tc 7c Th", // K T 9 7 A
            "8c Ks 4h 4d Ad Ts",    // K T 8 4 A
            "8c 5d 7c Ks As Ac 7d", // K 8 7 5 A
            "8d 4d 8s Ah 5h Kh",    // K 8 5 4 A
            "6d 6c Kh 7d 3s Ac",    // K 7 6 3 A
            "7h Kh Kc 5h Ah 2s",    // K 7 5 2 A
            "Qh 2c Kh 4c Jc 6c 4h", // Q J 6 4 2
            "Tc 7c Qd 6c 4s 7h",    // Q T 7 6 4
            "Tc 3d Qh Ad Ac Ks 7h", // Q T 7 3 A
            "4s Td 3d Tc Qs 5h 5c", // Q T 5 4 3
            "Qc Tc 2d 3c Ac",       // Q T 3 2 A
            "7h 4c 8c Qd 3h",       // Q 8 7 4 3
            "As 7s 4s Qs 3s Kc 7d", // Q 7 4 3 A
            "Qs Ks Ts Js 8s As 9s", // J T 9 8 A
            "9s Ad Jh Ts 2d",       // J T 9 2 A
            "Jd Ts Ks Ac 4c 8d 4s", // J T 8 4 A
            "5s Jc Td 2h 6h",       // J T 6 5 2
            "8h Jd 9s 3c 7d",       // J 9 8 7 3
            "6h 9d Ad 7s Jh 9c Jc", // J 9 7 6 A
            "8s 3d 8c Ah Js 7h",    // J 8 7 3 A
            "Js 2d 8d Qd 4c Ks 3h", // J 8 4 3 2
            "2d 6h 4s Js 6d 7d 6c", // J 7 6 4 2
            "5d Jc 3c As 4s 4h",    // J 5 4 3 A
            "2h 3d Tc 3h Ts 9s 8h", // T 9 8 3 2
            "4c Kc 9c Tc 6c 7c",    // T 9 7 6 4
            "7d Tc 5h 9c 2c",       // T 9 7 5 2
            "4d 9s 9h 6c 2s Ts",    // T 9 6 4 2
            "Js Th Jh 4c As 8s 6c", // T 8 6 4 A
            "7h Qs 2c 3s Ts 5d",    // T 7 5 3 2
            "Kd Td Ad 2h 3c Kh 6c", // T 6 3 2 A
            "Jd 8d 3d Jh Ah 9c 6h", // 9 8 6 3 A
            "Ac Qd 9c 8h 9h 5h 4s", // 9 8 5 4 A
            "9d 2h 6h 5h 7s Tc Jh", // 9 7 6 5 2
            "3d 9c Qh 4d Jc 5c 7c", // 9 7 5 4 3
            "Tc 3s Ts 4c 9d 2c 7d", // 9 7 4 3 2
            "Jd 9d 3h Tc 5h 6d As", // 9 6 5 3 A
            "Td 3h Jh 6c As 2d 9c", // 9 6 3 2 A
            "Jh 2s 7s Tc 4s 6c 8c", // 8 7 6 4 2
            "2d 7s 8c 9c 2c 6s Ah", // 8 7 6 2 A
            "5c 4d 8h Ah Qs 8s 6h", // 8 6 5 4 A
            "6s Tc 4h Ac 3c 6c 8h", // 8 6 4 3 A
            "As 8s 2c Qh 4s 6c 8d", // 8 6 4 2 A
            "5h 9h Jh 8s 2s 4d 3h", // 8 5 4 3 2
            "3h 5d Qd 4c Td 2d 7h", // 7 5 4 3 2
            "Qc 4h Ad 3c 7s 2d 9d", // 7 4 3 2 A
            "4d As 4s 5s 6h 2h Jc", // 6 5 4 2 A
            "3s 6d 9s 2d 5s Ad",    // 6 5 3 2 A
            "2s 3s Qh 5s 4s As Jc", // 5 4 3 2 A
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = ace_five_rank(&h1?);
            let r2 = ace_five_rank(&h2?);

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
