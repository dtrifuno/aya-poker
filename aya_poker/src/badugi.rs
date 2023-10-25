use aya_base::{constants::RANK_OFFSET, Hand, CARDS};

use crate::{insert_cards, BadugiRankCategory};

include!(concat!(env!("OUT_DIR"), "/badugi.rs"));

/// The strength ranking of a hand in Badugi.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct BadugiHandRank(pub u16);

/// Returns the rank of the best Badugi hand that can be made from the given cards.
///
/// # Examples
///
/// ```
/// use aya_poker::badugi_rank;
///
/// let hand = "Kc 4s 4h 8d".parse()?;
/// let rank = badugi_rank(&hand);
/// # Ok::<(), aya_poker::base::ParseError>(())
/// ```
pub fn badugi_rank(hand: &Hand) -> BadugiHandRank {
    let mut buffer = [CARDS[0]; 7];
    let cards = insert_cards(hand, &mut buffer);

    let k_max = usize::min(cards.len(), 4);
    let mut c = [0; 6];

    let mut rank = 1;
    for k in (1..=k_max).rev() {
        (0..k).for_each(|i| c[i] = i);
        c[k] = cards.len();
        c[k + 1] = 0;

        let mut j = 1;
        while j <= k {
            let subhand = (0..k).map(|i| cards[c[i]]).collect::<Hand>();
            if subhand.flush_count() == 1 {
                rank = rank.max(BADUGI_PHF.get(subhand.rank_key() as u64));
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

    BadugiHandRank(rank)
}

impl BadugiHandRank {
    /// Returns the Badugi hand rank category (i.e. number of valid cards in
    /// hand) that corresponds to the given Badugi hand rank.
    ///
    /// # Examples
    ///
    /// ```
    /// use aya_poker::{badugi_rank, BadugiRankCategory};
    ///
    /// let two_cards = badugi_rank(&"4c 4s Ac Js".parse()?);
    /// assert_eq!(two_cards.rank_category(), BadugiRankCategory::TwoCards);
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
    #[case::one_card("6c 6h 6s 6d", BadugiRankCategory::OneCard)]
    #[case::two_card("9c 4s Kc 4c", BadugiRankCategory::TwoCards)]
    #[case::three_card("Kc 3h 6d 7c", BadugiRankCategory::ThreeCards)]
    #[case::four_cards("Ac 2s 5d Kc Jh", BadugiRankCategory::FourCards)]
    fn rank_category(
        #[case] hand: &str,
        #[case] expected_category: BadugiRankCategory,
    ) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let ranking = badugi_rank(&hand);
        assert_eq!(ranking.rank_category(), expected_category);

        Ok(())
    }

    #[rstest]
    #[case::one_card(&[
        "Jh",
        "Jc",
        "Jh Jc Jd Js",
        "Jh Qh Kh",
        "Js Ks"
    ])]
    #[case::two_cards(&[
        "8c 6s Jc Qc Ks",
        "8c 8s 6c 6s Ks",
        "6c 8d Jc Td 9d",
        "8h 6s Kh Js 8s",
        "6c 6s 8s Kc Qs"
    ])]
    #[case::three_cards(&[
        "9c 7s 6d Jc Qc",
        "7d 6h 9s 8h Ts",
        "6c 9s 7d Kc Js Qd",
        "6h 7s 9c Th Qh",
        "7h 9c 6d 7c 7d"
    ])]
    #[case::four_cards(&[
        "5c 3s 2d Ah 5h",
        "2h 3d 5c As 3s",
        "2d 3s Ah 5c 5d",
        "3h 5c 2s Ad 3d",
        "5c Ah 3d 2s Ac Ad"
    ])]
    fn equal_rank_hands(#[case] hands: &[&str; 5]) -> Result<(), ParseError> {
        let hands = hands.map(|h| h.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = badugi_rank(&h1?);
            let r2 = badugi_rank(&h2?);

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
            // One card
            "",               // K
            "Qh Qd Qc Qs",    // Q
            "Js",             // J
            "8d",             // 8
            "7h 8h Qh",       // 7
            "Tc 4c 2c 7c Kc", // 2
            "8d Kd 5d Jd Ad", // A
            // Two cards
            "Kc Qh",          // K Q
            "Kc Ad Jd Kh",    // K A
            "Qh Js Qc",       // Q J
            "9h Jd",          // J 9
            "Jd 5h",          // J 5
            "2d Qd Js Kd Jh", // J 2
            "Th 8s",          // T 8
            "Th 7c 4c",       // T 4
            "2s Ts 2d",       // T 2
            "5s Jd 3s 9s 9d", // 9 3
            "2c 9c 9h Qc 5c", // 9 2
            "Qs Ts 9s 3h Ah", // 9 A
            "6s 8h Ts 8s",    // 8 6
            "8c 5h",          // 8 5
            "Ts Kh 7s 6h 5h", // 7 5
            "Jc Jd 3c 7d Tc", // 7 3
            "2h 7s",          // 7 2
            "7d Ac 2c 3c Kc", // 7 A
            "Jc Ts 6s 7s 5c", // 6 5
            "3c 8c Jc 6s Kc", // 6 3
            "5h 2s Ks",       // 5 2
            "Ad As 9s Ts 4s", // 4 A
            "2c 3s As 3c",    // 2 A
            // Three cards
            "Tc Kd Qs 5c",          // K Q 5
            "Js Kc 7h Qh",          // K J 7
            "Kd 4h 8s 8h",          // K 8 4
            "Jh 2h 3h 8c Ks",       // K 8 2
            "7d 4s Kc Qd 4h",       // K 7 4
            "Kc 9s Kh 3s 5h",       // K 5 3
            "Ad Kh 5c 7c Jc",       // K 5 A
            "Kc 3s Qd 4d",          // K 4 3
            "Qc 6h Td Tc",          // Q T 6
            "Ad Qs Th",             // Q T A
            "3s 6s 9h Qc 9s",       // Q 9 3
            "8s Qd 4h 9h",          // Q 8 4
            "7c Ah 9c 5h Qs",       // Q 7 5
            "6h 7h Qs 6c 3h",       // Q 6 3
            "Qd Tc 5s 6s 4c",       // Q 5 4
            "Qd Kh 5h 3s 8s",       // Q 5 3
            "4c Jc 3h Kd Qd",       // Q 4 3
            "6d Ts Jh Ks Kd",       // J T 6
            "5c 8c Jd Th Td",       // J T 5
            "Jh 9c 8s",             // J 9 8
            "Jh Kd Ts 8s 7d",       // J 8 7
            "6d 5c Js 3c Qd",       // J 6 3
            "Ac 8s Jc Jh Qc 4c 6s", // J 6 A
            "5c Jd 2s Qd",          // J 5 2
            "Td 5c 8d Ad Jh",       // J 5 A
            "2c Kh As 2h Jh",       // J 2 A
            "Tc 9s 8h 8c 9c",       // T 9 8
            "9d Jc 8s 4s Tc",       // T 9 4
            "Ah 8s Td",             // T 8 A
            "Ts 6c Th 7h",          // T 7 6
            "4h 6s Th 4d",          // T 6 4
            "Ts 3h Kh 4c 4s",       // T 4 3
            "As Td 7s 8c 2c",       // T 2 A
            "5s 9s Ac 9h Qc",       // 9 5 A
            "9d 3s 2h Kh 8s",       // 9 3 2
            "Jc 6s 8c 7d",          // 8 7 6
            "8d 9c 9d 4c 7h",       // 8 7 4
            "Td 6s Jd 8h 5d As",    // 8 5 A
            "8d 9d 2c 8c 4h",       // 8 4 2
            "6c 7d 2d 7h",          // 7 6 2
            "7d Ah 7h 5h 6c",       // 7 6 A
            "Td 8h Qh 7s 5d 3h Ks", // 7 5 3
            "2c 7d 5s Td 9s Tc 3c", // 7 5 2
            "9h 7c Ac 2h 7d",       // 7 2 A
            "4c 2s Ts 9s 4h Qh 6c", // 6 4 2
            "3c 2s 2h 5h",          // 5 3 2
            "Ad 8h 2s Ac Kh 4h 3s", // 4 2 A
            "9c As 3h 2c 6h",       // 3 2 A
            // Four cards
            "7s Jd Kc 9d Qh",       // K Q 9 7
            "4h Qs 4s 3c Kd",       // K Q 4 3
            "Ks 3c 2d Jd Qh",       // K Q 3 2
            "Ks 7d 6h 4s Kc",       // K 7 6 4
            "9s 3h Jc Qs Qc Qd",    // Q J 9 3
            "Jh 4d Qs 4h 2c",       // Q J 4 2
            "Qc 4d Ts 9h Jd",       // Q T 9 4
            "9s Qh 9d 7c 3d",       // Q 9 7 3
            "8d 3s 4c Qh",          // Q 8 4 3
            "3d 2s 6h Qc 7h",       // Q 6 3 2
            "6d Js 8c 2h 9c",       // J 8 6 2
            "6c Js 7h Ad 4h",       // J 6 4 A
            "4h Qs 8c Ts 2d",       // T 8 4 2
            "Td As Ts 4s Kc 8h 4c", // T 8 4 A
            "Qs Ks 6c 3d 8s Th 5s", // T 5 6 3
            "8h 6h 5d 4c 2d 7s",    // 7 6 4 2
            "8c 9d 7s 2d 2s 4c Ah", // 7 4 2 A
            "As 6c 4d 3h 5h 9h",    // 6 4 3 A
            "Js Jc 2d As 6h Tc 4c", // 6 4 2 A
            "4d Ac 5s 4c 9h 3h",    // 5 4 3 A
            "9d 5d 2h Ac 4s",       // 5 4 2 A
            "2c As 4d 3h 5s",       // 4 3 2 A
        ]
        .map(|s| s.parse());

        for (&h1, &h2) in hands.iter().zip(hands.iter().skip(1)) {
            let r1 = badugi_rank(&h1?);
            let r2 = badugi_rank(&h2?);

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
