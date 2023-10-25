use crate::Card;

/// Number of cards in a standard deck.
pub const CARD_COUNT: usize = 52;

/// Number of distinct ranks in a standard 52-card deck.
pub const RANK_COUNT: usize = 13;

/// Maximum number of cards that can be stored in a [`Hand`](crate::Hand).
pub const MAX_HAND_SIZE: usize = 7;

/// Offset of the suits counter in a [`Hand`][crate::Hand]'s `key` field.
pub const SUITS_SHIFT: usize = 48;

///
pub const RANK_OFFSET: usize = 4096;

pub(crate) const FLUSH_CHECK_MASK64: u64 = 0x8888 << SUITS_SHIFT;

pub(crate) const FLUSH_CHECK_MASK32: u32 = 0x8888 << (SUITS_SHIFT - 32);

/// Rank multipliers that guarantee a unique key for every rank combination
/// in a 0-7 card hand.
pub static RANKS: [u32; RANK_COUNT] = [
    0x2000, 0x8001, 0x11000, 0x3a000, 0x91000, 0x176005, 0x366000, 0x41a013, 0x47802e, 0x479068,
    0x48c0e4, 0x48f211, 0x494493,
];

pub static FLUSH_RANKS: [u32; 13] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];

/// All 52 standard playing cards in rank-suit order (deuces to aces,
/// clubs to spades).
pub static CARDS: [Card; CARD_COUNT] = {
    const CARD_COUNT_SHIFT: usize = 32;

    let mut data = [Card { key: 0, mask: 0 }; CARD_COUNT];
    let mut idx = 0;
    while idx < CARD_COUNT {
        let rank = idx / 4;
        let suit = idx % 4;

        let key = (1 << (4 * suit + SUITS_SHIFT)) + (1 << CARD_COUNT_SHIFT) + RANKS[rank] as u64;
        let mask = 1 << (16 * suit + rank);

        data[idx] = Card { key, mask };

        idx += 1;
    }

    data
};

/// Display string representations for all cards.
pub static CARDS_STR: [&str; CARD_COUNT] = [
    "2♣", "2♦", "2♥", "2♠", "3♣", "3♦", "3♥", "3♠", "4♣", "4♦", "4♥", "4♠", "5♣", "5♦", "5♥", "5♠",
    "6♣", "6♦", "6♥", "6♠", "7♣", "7♦", "7♥", "7♠", "8♣", "8♦", "8♥", "8♠", "9♣", "9♦", "9♥", "9♠",
    "T♣", "T♦", "T♥", "T♠", "J♣", "J♦", "J♥", "J♠", "Q♣", "Q♦", "Q♥", "Q♠", "K♣", "K♦", "K♥", "K♠",
    "A♣", "A♦", "A♥", "A♠",
];

/// Debug string representations for all cards.
pub static CARDS_DEBUG_STR: [&str; CARD_COUNT] = [
    "2c", "2d", "2h", "2s", "3c", "3d", "3h", "3s", "4c", "4d", "4h", "4s", "5c", "5d", "5h", "5s",
    "6c", "6d", "6h", "6s", "7c", "7d", "7h", "7s", "8c", "8d", "8h", "8s", "9c", "9d", "9h", "9s",
    "Tc", "Td", "Th", "Ts", "Jc", "Jd", "Jh", "Js", "Qc", "Qd", "Qh", "Qs", "Kc", "Kd", "Kh", "Ks",
    "Ac", "Ad", "Ah", "As",
];

/// Full English language names of all card ranks.
pub static RANK_NAMES: [&str; RANK_COUNT] = [
    "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen",
    "King", "Ace",
];

/// Full English language plural names of all card ranks.
pub static PLURAL_RANK_NAMES: [&str; RANK_COUNT] = [
    "Twos", "Threes", "Fours", "Fives", "Sixes", "Sevens", "Eights", "Nines", "Tens", "Jacks",
    "Queens", "Kings", "Aces",
];
