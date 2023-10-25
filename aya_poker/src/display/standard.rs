use aya_base::constants::{PLURAL_RANK_NAMES, RANK_NAMES, RANK_OFFSET};

use crate::{
    display::{conjunction, flush_suffix},
    PokerHandRank, PokerRankCategory,
};

const WORST_ACE_HIGH: usize = 785;
const WORST_KING_HIGH: usize = 456;
const WORST_QUEEN_HIGH: usize = 247;
const WORST_JACK_HIGH: usize = 122;
const WORST_TEN_HIGH: usize = 53;
const WORST_NINE_HIGH: usize = 19;
const WORST_EIGHT_HIGH: usize = 5;
const WORST_SEVEN_HIGH: usize = 1;

impl core::fmt::Display for PokerHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let determinant = self.0 as usize % RANK_OFFSET;
        let rc = self.rank_category();

        match rc {
            PokerRankCategory::Ineligible | PokerRankCategory::RoyalFlush => {
                write!(f, "{}", rc)
            }
            PokerRankCategory::HighCard | PokerRankCategory::Flush => {
                let r = match determinant {
                    WORST_ACE_HIGH.. => 12,
                    WORST_KING_HIGH.. => 11,
                    WORST_QUEEN_HIGH.. => 10,
                    WORST_JACK_HIGH.. => 9,
                    WORST_TEN_HIGH.. => 8,
                    WORST_NINE_HIGH.. => 7,
                    WORST_EIGHT_HIGH.. => 6,
                    WORST_SEVEN_HIGH.. => 5,
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
                let r = determinant + 2;
                write!(f, "{}, {}-high", rc, RANK_NAMES[r])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{base::ParseError, poker_rank};
    use rstest::rstest;

    #[rstest]
    #[case::high_card("9c 6s 5h 4h 2h", "High Card, Nine")]
    #[case::pair("6h Ah 6c 9s 8c", "Pair, Sixes")]
    #[case::two_pair("Ah 7c 4s 7d 4h", "Two Pair, Sevens and Fours")]
    #[case::three_of_a_kind("Jc Ah Js Kh Jd", "Three of a Kind, Jacks")]
    #[case::straight("2c Ah 3s 4h 5d 8s 8d", "Straight, Five-high")]
    #[case::flush("9s 7s 4s 3s 2s", "Flush, Nine-high")]
    #[case::full_house("Ks 6c Kc 6s 6d", "Full House, Sixes over Kings")]
    #[case::four_of_a_kinds("4c 6h 4s 4d 4h", "Four of a Kind, Fours")]
    #[case::straight_flush("9d 8d Jd Td 7d", "Straight Flush, Jack-high")]
    #[case::royal_flush("Ah Th Jh Kh Qh Ad", "Royal Flush")]
    fn poker_rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = poker_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }
}
