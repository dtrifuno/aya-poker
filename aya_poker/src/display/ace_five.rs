use aya_base::constants::{PLURAL_RANK_NAMES, RANK_NAMES, RANK_OFFSET};

use crate::{display::conjunction, AceFiveHandRank, PokerRankCategory};

const WORST_A_5_FIVE_HIGH: usize = 1287;
const WORST_A_5_SIX_HIGH: usize = 1282;
const WORST_A_5_SEVEN_HIGH: usize = 1267;
const WORST_A_5_EIGHT_HIGH: usize = 1232;
const WORST_A_5_NINE_HIGH: usize = 1162;
const WORST_A_5_TEN_HIGH: usize = 1036;
const WORST_A_5_JACK_HIGH: usize = 826;
const WORST_A_5_QUEEN_HIGH: usize = 496;
const WORST_A_5_KING_HIGH: usize = 1;

fn from_ace_five_index(r: usize) -> usize {
    if r == 12 {
        12
    } else {
        11 - r
    }
}

impl core::fmt::Display for AceFiveHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let determinant = self.0 as usize % RANK_OFFSET;
        let rc = self.rank_category();

        match rc {
            PokerRankCategory::FourOfAKind
            | PokerRankCategory::ThreeOfAKind
            | PokerRankCategory::Pair => {
                let r = from_ace_five_index(determinant / 256);
                write!(f, "{}, {}", rc, PLURAL_RANK_NAMES[r])
            }
            PokerRankCategory::TwoPair | PokerRankCategory::FullHouse => {
                let r1 = from_ace_five_index(determinant / 256);
                let r2 = from_ace_five_index((determinant % 256) / 16);

                write!(
                    f,
                    "{}, {} {} {}",
                    rc,
                    PLURAL_RANK_NAMES[r1],
                    conjunction(rc),
                    PLURAL_RANK_NAMES[r2]
                )
            }
            PokerRankCategory::HighCard => {
                let r = match determinant {
                    WORST_A_5_FIVE_HIGH.. => 3,
                    WORST_A_5_SIX_HIGH.. => 4,
                    WORST_A_5_SEVEN_HIGH.. => 5,
                    WORST_A_5_EIGHT_HIGH.. => 6,
                    WORST_A_5_NINE_HIGH.. => 7,
                    WORST_A_5_TEN_HIGH.. => 8,
                    WORST_A_5_JACK_HIGH.. => 9,
                    WORST_A_5_QUEEN_HIGH.. => 10,
                    WORST_A_5_KING_HIGH.. => 11,
                    _ => unreachable!(),
                };
                write!(f, "{}, {}", rc, RANK_NAMES[r])
            }
            PokerRankCategory::Ineligible => write!(f, "{}", rc),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ace_five_rank, base::ParseError};
    use rstest::rstest;

    #[rstest]
    #[case::four_of_a_kind("Jh Jc Jd Js 5h", "Four of a Kind, Jacks")]
    #[case::full_house("5c As 5d Ah 5s", "Full House, Fives over Aces")]
    #[case::three_of_a_kind("3h 2h Ah Ac Ad", "Three of a Kind, Aces")]
    #[case::two_pair("4c 9s 9d 4h Ac", "Two Pair, Nines and Fours")]
    #[case::two_pair("6c As 5d 6h Ac", "Two Pair, Sixes and Aces")]
    #[case::pair("Jc Js Ah Kh Qh", "Pair, Jacks")]
    #[case::high_card("Ah 5c 4s 3d 2h", "High Card, Five")]
    fn rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = ace_five_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }
}
