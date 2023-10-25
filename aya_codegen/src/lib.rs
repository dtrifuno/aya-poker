mod ace_five;
mod baduci;
mod badugi;
mod deuce_seven;
mod six_plus;
mod standard;
pub(crate) mod utils;

pub use ace_five::AceFiveLowballLookup;
pub use baduci::BaduciLookup;
pub use badugi::BadugiLookup;
pub use deuce_seven::DeuceSevenLowballLookup;
pub use six_plus::SixPlusPokerLookup;
pub use standard::PokerLookup;

const HAND_CATEGORY_OFFSET: u16 = 0x1000;
