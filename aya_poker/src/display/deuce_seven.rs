use aya_base::constants::{PLURAL_RANK_NAMES, RANK_NAMES, RANK_OFFSET};

use crate::{
    display::{conjunction, flush_suffix},
    DeuceSevenHandRank, PokerRankCategory,
};

const WORST_2_7_SEVEN_HIGH: usize = 1274;
const WORST_2_7_EIGHT_HIGH: usize = 1260;
const WORST_2_7_NINE_HIGH: usize = 1226;
const WORST_2_7_TEN_HIGH: usize = 1157;
const WORST_2_7_JACK_HIGH: usize = 1032;
const WORST_2_7_QUEEN_HIGH: usize = 823;
const WORST_2_7_KING_HIGH: usize = 494;
const WORST_2_7_ACE_HIGH: usize = 1;

impl core::fmt::Display for DeuceSevenHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let determinant = self.0 as usize % RANK_OFFSET;
        let rc = self.rank_category();

        match rc {
            PokerRankCategory::Ineligible | PokerRankCategory::RoyalFlush => {
                write!(f, "{}", rc)
            }
            PokerRankCategory::HighCard | PokerRankCategory::Flush => {
                let r = match determinant {
                    WORST_2_7_SEVEN_HIGH.. => 5,
                    WORST_2_7_EIGHT_HIGH.. => 6,
                    WORST_2_7_NINE_HIGH.. => 7,
                    WORST_2_7_TEN_HIGH.. => 8,
                    WORST_2_7_JACK_HIGH.. => 9,
                    WORST_2_7_QUEEN_HIGH.. => 10,
                    WORST_2_7_KING_HIGH.. => 11,
                    WORST_2_7_ACE_HIGH.. => 12,
                    _ => unreachable!(),
                };
                write!(f, "{}, {}{}", rc, RANK_NAMES[r], flush_suffix(rc))
            }
            PokerRankCategory::Pair
            | PokerRankCategory::ThreeOfAKind
            | PokerRankCategory::FourOfAKind => {
                let r = determinant / 256;
                write!(f, "{}, {}", rc, PLURAL_RANK_NAMES[12 - r])
            }
            PokerRankCategory::TwoPair | PokerRankCategory::FullHouse => {
                let r1 = determinant / 256;
                let r2 = (determinant % 256) / 16;
                write!(
                    f,
                    "{}, {} {} {}",
                    rc,
                    PLURAL_RANK_NAMES[12 - r1],
                    conjunction(rc),
                    PLURAL_RANK_NAMES[12 - r2]
                )
            }
            PokerRankCategory::Straight | PokerRankCategory::StraightFlush => {
                let r = 13 - determinant;
                write!(f, "{}, {}-high", rc, RANK_NAMES[r])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{base::ParseError, deuce_seven_rank};
    use rstest::rstest;

    #[rstest]
    #[case::royal_flush("Ts Js Qs Ks As", "Royal Flush")]
    #[case::straight_flush("5c 8c 6c 7c 4c", "Straight Flush, Eight-high")]
    #[case::four_of_a_kind("8c 5s 8d 8h 8s", "Four of a Kind, Eights")]
    #[case::full_house("7c 7s Ad Ah 7h", "Full House, Sevens over Aces")]
    #[case::flush("9h 3h 5h 6h 2h", "Flush, Nine-high")]
    #[case::straight("Ac 5s 3d 2h 4h", "Straight, Five-high")]
    #[case::three_of_a_kind("Jc 5h Js 3h Jd", "Three of a Kind, Jacks")]
    #[case::two_pair("7c 7s 3c 3s Ks", "Two Pair, Sevens and Threes")]
    #[case::pair("5h 5c As 7s 3s", "Pair, Fives")]
    #[case::high_card("Jh 7s 5d 4d 2c", "High Card, Jack")]
    fn deuce_seven_rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = deuce_seven_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }
}
