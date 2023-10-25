use core::{convert::TryFrom, str::FromStr};

use super::card::ParseError;

/// One of the four French playing card suits.
#[derive(PartialEq, Eq, Debug)]
pub enum Suit {
    Clubs = 0,
    Diamonds,
    Hearts,
    Spades,
}

impl TryFrom<u8> for Suit {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Clubs),
            1 => Ok(Suit::Diamonds),
            2 => Ok(Suit::Hearts),
            3 => Ok(Suit::Spades),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for Suit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "c" | "♣" => Ok(Suit::Clubs),
            "d" | "♦" => Ok(Suit::Diamonds),
            "h" | "♥" => Ok(Suit::Hearts),
            "s" | "♠" => Ok(Suit::Spades),
            _ => Err(ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("c", Ok(Suit::Clubs))]
    #[case("h", Ok(Suit::Hearts))]
    #[case("♦", Ok(Suit::Diamonds))]
    #[case("♠", Ok(Suit::Spades))]
    #[case("a", Err(ParseError))]
    #[case("H", Err(ParseError))]
    fn parse(#[case] s: &str, #[case] expected: Result<Suit, ParseError>) {
        let result = s.parse::<Suit>();
        assert_eq!(result, expected);
    }
}
