use aya_base::{constants::RANK_OFFSET, Hand, CARDS};

use crate::{insert_cards, BadugiRankCategory};

include!(concat!(env!("OUT_DIR"), "/baduci.rs"));

/// The strength ranking of a hand in Baduci (ace-high Badugi).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct BaduciHandRank(pub u16);

/// Returns the rank of the best Baduci (Badugi with aces playing high) hand
/// that can be made from the given cards.
///
/// Note that this only returns the ranking of the Badugi hand&mdash;use
/// [`deuce_seven_rank`](crate::deuce_seven_rank) to get the deuce to seven
/// lowball ranking as well.
///
/// # Examples
///
/// ```
/// use aya_poker::baduci_rank;
///
/// let hand = "Ac 3h 5s Kh".parse()?;
/// let rank = baduci_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
pub fn baduci_rank(hand: &Hand) -> BaduciHandRank {
    let mut buffer = [CARDS[0]; 7];
    let cards = insert_cards(hand, &mut buffer);

    let mut rank = 1;
    let k_max = usize::min(cards.len(), 4);
    let mut c = [0; 6];

    for k in (1..=k_max).rev() {
        (0..k).for_each(|i| c[i] = i);
        c[k] = cards.len();
        c[k + 1] = 0;

        let mut j = 1;
        while j <= k {
            let subhand = (0..k).map(|i| cards[c[i]]).collect::<Hand>();
            if subhand.flush_count() == 1 {
                rank = rank.max(BADUCI_PHF.get(subhand.rank_key() as u64));
            }

            j = 1;
            while c[j - 1] + 1 == c[j] {
                c[j - 1] = j - 1;
                j += 1;
            }

            c[j - 1] += 1;
        }

        if rank > 1 {
            break;
        }
    }

    BaduciHandRank(rank)
}

impl BaduciHandRank {
    /// Returns the Badugi hand rank category (i.e. number of valid cards in
    /// hand) that corresponds to the given Baduci hand rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use aya_poker::{baduci_rank, BadugiRankCategory};
    ///
    /// let three_cards = baduci_rank(&"Js 5c 8s 4d Jc".parse()?);
    /// assert_eq!(three_cards.rank_category(), BadugiRankCategory::ThreeCards);
    /// # Ok::<(), aya_poker::base::ParseError>(())
    /// ```
    pub fn rank_category(&self) -> BadugiRankCategory {
        match self.0 as usize / RANK_OFFSET {
            0 => BadugiRankCategory::OneCard,
            1 => BadugiRankCategory::TwoCards,
            2 => BadugiRankCategory::ThreeCards,
            3 => BadugiRankCategory::FourCards,
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
    #[case::one_card("Ah 5h 8h Kh", BadugiRankCategory::OneCard)]
    #[case::two_card("7h 5h Ad Ac", BadugiRankCategory::TwoCards)]
    #[case::three_card("Jh 3c Qd 2d", BadugiRankCategory::ThreeCards)]
    #[case::four_cards("Kh As 3d 8c", BadugiRankCategory::FourCards)]
    fn rank_category(
        #[case] hand: &str,
        #[case] expected_category: BadugiRankCategory,
    ) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let ranking = baduci_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::one_card(&[
        "6h",
        "6c",
        "6h 6c 6s 6d",
        "6h 8h 9h Kh Th",
        "6d 9d Ad Kd Jd"
    ])]
    #[case::two_cards(&[
        "7h 6c",
        "7c Ac 6h Kc Tc",
        "7c 7s 6d 6h",
        "7h 6c 6s 6d 6h",
        "7c 6s Kc Qs Jc"
    ])]
    #[case::three_cards(&[
        "7c 5h 3s",
        "7d 5h 3s 9d Jh",
        "7c 7h 5h 3s 5s",
        "7c 7h 7d 7s 5c 5h 3d",
        "7c Kc Ac 5h Jh 3s"
    ])]
    #[case::four_cards(&[
        "Ac 9s 6h 4d",
        "Ah 4s 6d 9c Ac",
        "As 9d 6h 4c 5c",
        "Ad 4s 6h 9c Jc",
        "Ac 9s 6d 4h 6c"
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = baduci_rank(&h1?);
            let r2 = baduci_rank(&h2?);

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
            // One cards
            "",      // A
            "Ks",    // K
            "Jd",    // J
            "Td",    // T
            "8c",    // 8
            "7c",    // 7
            "2c 2d", // 2
            // Two cards
            "Ts Tc Ad",          // A T
            "8s Ac Qs",          // A 8
            "Ah Ac 7h 8h Ad",    // A 7
            "6h 9h Ac 2h 5h",    // A 2
            "Jh Kc",             // K J
            "Ks 3h",             // K 3
            "9h Kh Kd 2h",       // K 2
            "9d Jd Qc Qd Ad",    // Q 9
            "7d Qh",             // Q 7
            "3h Qs",             // Q 3
            "Kc 2c 3c Qh Jc",    // Q 2
            "6d Jc 8d",          // J 6
            "4s Js Jd 5s",       // J 4
            "Th 7c 6c 2c",       // T 2
            "8h Jh Jd Td 7d",    // 8 7
            "7s 5c Qs Tc",       // 7 5
            "7c 2s 8s 2c",       // 7 2
            "Ah 8c 3h 6c",       // 6 3
            "4h 8c 5c Qh Qc",    // 5 4
            "2d 8d 2c 5d",       // 5 2
            "5c 2s Qc 9c 4c As", // 4 2
            "3d 2c Jd",          // 3 2
            // Three cards
            "Qs Ad Js Kh",          // A K J
            "3h Tc Ad",             // A T 3
            "9h Kh Th As 3c Kc Ah", // A 9 3
            "As 9d 2c",             // A 9 2
            "3c 7c Qc 8h Ad",       // A 8 3
            "Kd Ac 7s 3d Js 6d 4d", // A 7 3
            "Ah 5d 5h 4c",          // A 5 4
            "Ac Qs Kh 5s 8c",       // K 8 5
            "8d Ks 4h Ah",          // K 8 4
            "6h Ad Kd 7s",          // K 7 6
            "Qc 2c 6d Kh",          // K 6 2
            "2s 5c Kd Ad 5h",       // K 5 2
            "Js As 4c Qh",          // Q J 4
            "Qh 2d Ad Jc Kh",       // Q J 2
            "Ks 9s Td Qc",          // Q T 9
            "5s 5d Th Qd",          // Q T 5
            "2s Qd Th 5s Qc",       // Q T 2
            "Kd 8s 9d Qd 9c",       // Q 9 8
            "Qs 4h 8d",             // Q 8 4
            "5h As 7d Qs",          // Q 7 5
            "5h Qs 6h Tc 3c",       // Q 5 3
            "Td Jh 9s Kd",          // J T 9
            "2h 5h 4h Tc 9h Js",    // J T 2
            "Kd 8s As Jd 9h",       // J 9 8
            "7d 8s 7s Jh Ks",       // J 8 7
            "3s 3c Jh 5d Kh",       // J 5 3
            "Js 5s Jh Jc 6d 9d 2d", // J 5 2
            "5s 7s 4h Jd 3s",       // J 4 2
            "9h 5s 2h Jc 4s",       // J 4 3
            "4s 8h 8s Tc Kc",       // T 8 4
            "Kd 4s Tc 7d 4c",       // T 7 4
            "Qs 5c Jc Jh 6c Ts 6h", // T 6 5
            "Qd 6c Ts 4d 6d",       // T 6 4
            "Qh Kc 9h Kd 8c 2d 7d", // 9 8 2
            "7d 2h 6d 9c 8d",       // 9 6 2
            "3s 9h 2d 3h 6d",       // 9 6 3
            "Kh Jh 6h 8d 7c",       // 8 7 6
            "Qh 5s 7h 8d",          // 8 7 5
            "7s 6d 8c 4s",          // 8 6 4
            "9d 6d 6c 8d 2s",       // 8 6 2
            "6d 8c 3d 8s 5c",       // 8 5 3
            "5h 8s 2h Ah 7h 5d",    // 8 5 2
            "5h 2h 3c 8s Ac Th",    // 8 3 2
            "6d 3d Jd 7h 5c",       // 7 5 3
            "8s 4h 7s 3d Th",       // 7 4 3
            "3h 7c 6c 4s Qc",       // 6 4 3
            "9c Qc 3h 4s 7h 5c As", // 5 4 3
            // Four cards
            "Ks 4h Jc Jd Ac",       // A K J 4
            "9d 6c Ah 9h 8c Qd 9s", // A Q 9 6
            "Ac 4s 4c 7h Td",       // A T 7 4
            "2c 7c 9h 7d 9d As 2h", // A 9 7 2
            "Ah 6c 5d Ts 2s",       // A 6 5 2
            "9c 4s Qs 6h Kd",       // K 9 6 4
            "5d Kc Ah Ts 7d 8s 7h", // K 8 7 5
            "Kd 2s Ac 6h 7c",       // K 7 6 2
            "Ks 6d Td 4h 5c",       // K 6 5 4
            "9s Qc 8d Jh",          // Q J 9 8
            "7s 8c Qd 2h",          // Q 8 7 2
            "8h Qd 6s 3c Jc Js",    // Q 8 6 3
            "Qs 2s 4c 7h Jh 6c 2d", // Q 7 4 2
            "4h 3d Jc Qs 9d 5c",    // Q 5 4 3
            "5s Qc 2d 3h Qd 2s 4d", // Q 4 3 2
            "9c 5d 9d Th Js",       // J T 9 5
            "8h Kh Ad 5c Jd 5d 3s", // J 8 5 3
            "Th 8s 9d 4c",          // T 9 8 4
            "Ac Kd 8d Td 2s 8h 6c", // T 8 6 2
            "Qc Ts 8d 2h 7d 6c",    // T 7 6 2
            "7s Th Ah 2d 4c 2h",    // T 7 4 2
            "5c Qs Ad 8h 2s 2d 9d", // 9 8 5 2
            "9c 4h Qd 7s 6d",       // 9 7 6 4
            "6s 2h Qh 4c Ts 7d",    // 7 6 4 2
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = baduci_rank(&h1?);
            let r2 = baduci_rank(&h2?);

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
