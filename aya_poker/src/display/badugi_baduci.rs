use aya_base::constants::{RANK_NAMES, RANK_OFFSET};

use crate::{BaduciHandRank, BadugiHandRank, BadugiRankCategory};

const WORST_BADUGI_2_CARD_TWO: usize = 78;
const WORST_BADUGI_2_CARD_THREE: usize = 76;
const WORST_BADUGI_2_CARD_FOUR: usize = 73;
const WORST_BADUGI_2_CARD_FIVE: usize = 69;
const WORST_BADUGI_2_CARD_SIX: usize = 64;
const WORST_BADUGI_2_CARD_SEVEN: usize = 58;
const WORST_BADUGI_2_CARD_EIGHT: usize = 51;
const WORST_BADUGI_2_CARD_NINE: usize = 43;
const WORST_BADUGI_2_CARD_TEN: usize = 34;
const WORST_BADUGI_2_CARD_JACK: usize = 24;
const WORST_BADUGI_2_CARD_QUEEN: usize = 13;
const WORST_BADUGI_2_CARD_KING: usize = 1;

const WORST_BADUGI_3_CARD_THREE: usize = 286;
const WORST_BADUGI_3_CARD_FOUR: usize = 283;
const WORST_BADUGI_3_CARD_FIVE: usize = 277;
const WORST_BADUGI_3_CARD_SIX: usize = 267;
const WORST_BADUGI_3_CARD_SEVEN: usize = 252;
const WORST_BADUGI_3_CARD_EIGHT: usize = 231;
const WORST_BADUGI_3_CARD_NINE: usize = 203;
const WORST_BADUGI_3_CARD_TEN: usize = 167;
const WORST_BADUGI_3_CARD_JACK: usize = 122;
const WORST_BADUGI_3_CARD_QUEEN: usize = 67;
const WORST_BADUGI_3_CARD_KING: usize = 1;

const WORST_BADUGI_4_CARD_FOUR: usize = 715;
const WORST_BADUGI_4_CARD_FIVE: usize = 711;
const WORST_BADUGI_4_CARD_SIX: usize = 701;
const WORST_BADUGI_4_CARD_SEVEN: usize = 681;
const WORST_BADUGI_4_CARD_EIGHT: usize = 646;
const WORST_BADUGI_4_CARD_NINE: usize = 590;
const WORST_BADUGI_4_CARD_TEN: usize = 506;
const WORST_BADUGI_4_CARD_JACK: usize = 386;
const WORST_BADUGI_4_CARD_QUEEN: usize = 221;
const WORST_BADUGI_4_CARD_KING: usize = 1;

impl core::fmt::Display for BadugiHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rc = self.rank_category();
        let determinant = self.0 as usize % RANK_OFFSET;

        display_badugi_style_rank(rc, determinant, |r| if r == 12 { 12 } else { 11 - r }, f)
    }
}

impl core::fmt::Display for BaduciHandRank {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let rc = self.rank_category();
        let determinant = self.0 as usize % RANK_OFFSET;

        display_badugi_style_rank(rc, determinant, |r| 12 - r, f)
    }
}

fn suffix(rc: BadugiRankCategory) -> &'static str {
    match rc {
        BadugiRankCategory::OneCard => "",
        _ => "-high",
    }
}

fn display_badugi_style_rank(
    rc: BadugiRankCategory,
    determinant: usize,
    lowball_idx: impl Fn(usize) -> usize,
    f: &mut core::fmt::Formatter<'_>,
) -> core::fmt::Result {
    let i = match rc {
        BadugiRankCategory::OneCard => determinant - 1,
        BadugiRankCategory::TwoCards => match determinant {
            WORST_BADUGI_2_CARD_TWO.. => 11,
            WORST_BADUGI_2_CARD_THREE.. => 10,
            WORST_BADUGI_2_CARD_FOUR.. => 9,
            WORST_BADUGI_2_CARD_FIVE.. => 8,
            WORST_BADUGI_2_CARD_SIX.. => 7,
            WORST_BADUGI_2_CARD_SEVEN.. => 6,
            WORST_BADUGI_2_CARD_EIGHT.. => 5,
            WORST_BADUGI_2_CARD_NINE.. => 4,
            WORST_BADUGI_2_CARD_TEN.. => 3,
            WORST_BADUGI_2_CARD_JACK.. => 2,
            WORST_BADUGI_2_CARD_QUEEN.. => 1,
            WORST_BADUGI_2_CARD_KING.. => 0,
            _ => unreachable!(),
        },
        BadugiRankCategory::ThreeCards => match determinant {
            WORST_BADUGI_3_CARD_THREE.. => 10,
            WORST_BADUGI_3_CARD_FOUR.. => 9,
            WORST_BADUGI_3_CARD_FIVE.. => 8,
            WORST_BADUGI_3_CARD_SIX.. => 7,
            WORST_BADUGI_3_CARD_SEVEN.. => 6,
            WORST_BADUGI_3_CARD_EIGHT.. => 5,
            WORST_BADUGI_3_CARD_NINE.. => 4,
            WORST_BADUGI_3_CARD_TEN.. => 3,
            WORST_BADUGI_3_CARD_JACK.. => 2,
            WORST_BADUGI_3_CARD_QUEEN.. => 1,
            WORST_BADUGI_3_CARD_KING.. => 0,
            _ => unreachable!(),
        },
        BadugiRankCategory::FourCards => match determinant {
            WORST_BADUGI_4_CARD_FOUR.. => 9,
            WORST_BADUGI_4_CARD_FIVE.. => 8,
            WORST_BADUGI_4_CARD_SIX.. => 7,
            WORST_BADUGI_4_CARD_SEVEN.. => 6,
            WORST_BADUGI_4_CARD_EIGHT.. => 5,
            WORST_BADUGI_4_CARD_NINE.. => 4,
            WORST_BADUGI_4_CARD_TEN.. => 3,
            WORST_BADUGI_4_CARD_JACK.. => 2,
            WORST_BADUGI_4_CARD_QUEEN.. => 1,
            WORST_BADUGI_4_CARD_KING.. => 0,
            _ => unreachable!(),
        },
    };
    let r = lowball_idx(i);
    write!(f, "{}, {}{}", rc, RANK_NAMES[r], suffix(rc))
}

#[cfg(test)]
mod tests {
    use crate::{baduci_rank, badugi_rank, base::ParseError};
    use rstest::rstest;

    #[rstest]
    #[case::one_card("Ad Ah", "One Card, Ace")]
    #[case::one_card("2d 4d 6d", "One Card, Two")]
    #[case::two_cards("Qd Ad 7h 7d", "Two Cards, Queen-high")]
    #[case::two_cards("9c 6c 9s 6s", "Two Cards, Nine-high")]
    #[case::two_cards("7c 6s 3c 9s", "Two Cards, Six-high")]
    #[case::three_cards("Qh Qc 5s 6d", "Three Cards, Queen-high")]
    #[case::three_cards("8d 6c 4s", "Three Cards, Eight-high")]
    #[case::four_cards("Kh Qc As 3d", "Four Cards, Ace-high")]
    #[case::four_cards("3s 9d 5c Th", "Four Cards, Ten-high")]
    #[case::four_cards("3c 2s 5h 4d", "Four Cards, Five-high")]
    fn baduci_rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = baduci_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }

    #[rstest]
    #[case::one_card("6d 8d Jd", "One Card, Six")]
    #[case::one_card("Ad Ah Ac As", "One Card, Ace")]
    #[case::two_cards("Qd Jd 7h 7c", "Two Cards, Jack-high")]
    #[case::two_cards("7c 6c 7s 6s", "Two Cards, Seven-high")]
    #[case::two_cards("7c 5s 3c 2s", "Two Cards, Three-high")]
    #[case::three_cards("Qh Qc 5s 6d", "Three Cards, Queen-high")]
    #[case::three_cards("8d 6c 4s", "Three Cards, Eight-high")]
    #[case::four_cards("Kh Qc 4s 3d", "Four Cards, King-high")]
    #[case::four_cards("7c 8s 6d Th", "Four Cards, Ten-high")]
    #[case::four_cards("2c 3s 6d 5h", "Four Cards, Six-high")]
    fn badugi_rank_name(#[case] hand: &str, #[case] expected: &str) -> Result<(), ParseError> {
        let hand = hand.parse()?;
        let rank = badugi_rank(&hand);
        assert_eq!(&rank.to_string(), expected);
        Ok(())
    }
}
