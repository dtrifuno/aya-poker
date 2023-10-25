use aya_base::constants::{PLURAL_RANK_NAMES, RANK_NAMES, RANK_OFFSET};

use crate::{
    display::{conjunction, flush_suffix},
    PokerRankCategory, ShortDeckHandRank,
};

const WORST_6_PLUS_ACE_HIGH: usize = 54;
const WORST_6_PLUS_KING_HIGH: usize = 19;
const WORST_6_PLUS_QUEEN_HIGH: usize = 5;
const WORST_6_PLUS_JACK_HIGH: usize = 1;

impl core::fmt::Display for ShortDeckHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let determinant = self.0 as usize % RANK_OFFSET;
        let rc = self.rank_category();

        match rc {
            PokerRankCategory::Ineligible | PokerRankCategory::RoyalFlush => {
                write!(f, "{}", rc)
            }
            PokerRankCategory::HighCard | PokerRankCategory::Flush => {
                let r = match determinant {
                    WORST_6_PLUS_ACE_HIGH.. => 12,
                    WORST_6_PLUS_KING_HIGH.. => 11,
                    WORST_6_PLUS_QUEEN_HIGH.. => 10,
                    WORST_6_PLUS_JACK_HIGH.. => 9,
                    _ => unreachable!(),
                };
                write!(f, "{}, {}{}", rc, RANK_NAMES[r], flush_suffix(rc))
            }
            PokerRankCategory::Pair
            | PokerRankCategory::ThreeOfAKind
            | PokerRankCategory::FourOfAKind => {
                let r = determinant / 256;
                write!(f, "{}, {}", rc, PLURAL_RANK_NAMES[r])
            }
            PokerRankCategory::TwoPair | PokerRankCategory::FullHouse => {
                let r1 = determinant / 256;
                let r2 = (determinant % 256) / 16;
                write!(
                    f,
                    "{}, {} {} {}",
                    rc,
                    PLURAL_RANK_NAMES[r1],
                    conjunction(rc),
                    PLURAL_RANK_NAMES[r2]
                )
            }
            PokerRankCategory::Straight | PokerRankCategory::StraightFlush => {
                let r = determinant + 6;
                write!(f, "{}, {}-high", rc, RANK_NAMES[r])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{base::ParseError, short_deck_rank};
    use rstest::rstest;

    #[rstest]
    #[case::high_card("Js 6c 9h 8d", "High Card, Jack")]
    #[case::flush("Qc 6h 7c Tc 9c", "High Card, Queen")]
    #[case::pair("7h 8s 9s 6s 7c", "Pair, Sevens")]
    #[case::two_pair("Jc 7c Js 7s As", "Two Pair, Jacks and Sevens")]
    #[case::three_of_a_kind("Qc As Qd Kh Qh", "Three of a Kind, Queens")]
    #[case::straight("6h Ac 7s 9c 8c", "Straight, Nine-high")]
    #[case::full_house("8c Qs Qd 8d 8h", "Full House, Eights over Queens")]
    #[case::flush("Qc 6c 7c Tc 9c", "Flush, Queen-high")]
    #[case::four_of_a_kind("Tc Ts Ac Td Th", "Four of a Kind, Tens")]
    #[case::straight_flush("Td 9d 8d 7d 6d", "Straight Flush, Ten-high")]
    #[case::royal_flush("Qc Jc Ac Kc Tc", "Royal Flush")]
    fn rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = short_deck_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }
}
