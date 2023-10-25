mod ace_five;
mod badugi_baduci;
mod deuce_seven;
mod six_plus;
mod standard;

use crate::PokerRankCategory;

fn flush_suffix(rc: PokerRankCategory) -> &'static str {
    match rc {
        PokerRankCategory::HighCard => "",
        PokerRankCategory::Flush => "-high",
        _ => unreachable!(),
    }
}

fn conjunction(rc: PokerRankCategory) -> &'static str {
    match rc {
        PokerRankCategory::TwoPair => "and",
        PokerRankCategory::FullHouse => "over",
        _ => unreachable!(),
    }
}
