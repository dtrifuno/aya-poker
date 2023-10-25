use core::{convert::TryFrom, str::FromStr};

use super::card::ParseError;

/// One of the thirteen ranks of a standard French 52-playing card deck.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Rank {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<u8> for Rank {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::Two),
            1 => Ok(Rank::Three),
            2 => Ok(Rank::Four),
            3 => Ok(Rank::Five),
            4 => Ok(Rank::Six),
            5 => Ok(Rank::Seven),
            6 => Ok(Rank::Eight),
            7 => Ok(Rank::Nine),
            8 => Ok(Rank::Ten),
            9 => Ok(Rank::Jack),
            10 => Ok(Rank::Queen),
            11 => Ok(Rank::King),
            12 => Ok(Rank::Ace),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for Rank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError)?;
        }

        let c = s.chars().next().unwrap();
        let idx = match c {
            '2'..='9' => c.to_digit(10).unwrap() as u8 - 2,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => Err(ParseError)?,
        };
        Self::try_from(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("A", Ok(Rank::Ace))]
    #[case("J", Ok(Rank::Jack))]
    #[case("9", Ok(Rank::Nine))]
    #[case("2", Ok(Rank::Two))]
    #[case("a", Err(ParseError))]
    #[case("t", Err(ParseError))]
    fn parse(#[case] s: &str, #[case] expected: Result<Rank, ParseError>) {
        let result = s.parse::<Rank>();
        assert_eq!(result, expected);
    }
}
