# AyaPoker

[![Latest Release]][crates.io] [![Documentation]][docs.rs] ![Minimum Supported Rust Version 1.64]

[Latest Release]: https://img.shields.io/crates/v/aya_poker.svg
[crates.io]: https://crates.io/crates/aya_poker
[Documentation]: https://docs.rs/aya_poker/badge.svg
[docs.rs]: https://docs.rs/aya_poker/
[Minimum Supported Rust Version 1.64]: https://img.shields.io/badge/MSRV-1.64-blue.svg

AyaPoker is a Rust library for fast poker hand evaluation based on the
[OMPEval C++ hand evaluator](https://github.com/zekyll/OMPEval), with support for most
popular poker variants.

## Features

- Can be used to rank hands from standard poker, ace-to-five lowball, deuce-to-seven
  lowball, six-or-better (short-deck), Omaha, Omaha Hi/Lo, Badugi or Baduci.
- Can evaluate hands with 0 to 7 cards, with the missing cards counting as
  the worst possible kickers, allowing for use in stud poker games.
- Uses compile-time generated perfect hash function lookup tables for excellent
  runtime performance and fast initialization.
- Has extensive suite of tests to ensure correct implementation of the hand ranking
  rules for each variant.

## Flags

This crate has the following Cargo features:

- `std`: By default, `aya_poker` is a `!#[no_std]` crate, but can be compiled
  with the `std` feature in order to allow the initialization of `Deck`s
  with system-generated random seeds.
- `colored`: Use [`colored`](https://crates.io/crates/colored) to display
  cards and hands in color.
- `colored-4color`: Same as `colored`, but using a four-color deck.

## Example

```rust
use std::cmp::Ordering;

use aya_poker::{base::*, deck::Deck, poker_rank};

const SEED: u64 = 42;
const SAMPLE_SIZE: usize = 100_000;

fn main() -> Result<(), ParseError> {
    // We can initialize cards by specifying the rank and suit,
    // and then use collect to combine the cards into a Hand
    let player = [
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::King, Suit::Clubs),
    ]
    .iter()
    .collect();

    // Or, first parse the cards from strings
    let opponent = ["Js".parse::<Card>(), "Qs".parse::<Card>()]
        .iter()
        .copied()
        .collect::<Result<Hand, ParseError>>()?;

    // Or, parse a space-separated string of cards as a Hand
    let board = "Ah 5s Ts".parse()?;

    let equity = equity_calculator(&player, &opponent, &board);
    println!(
        "{} has {:.1}% equity on {} against {}.",
        player,
        100.0 * equity,
        board,
        opponent
    );

    Ok(())

}

fn equity_calculator(
    player: &Hand,
    opponent: &Hand,
    board: &Hand,
) -> f64 {
    // To simulate board run-outs, we begin by preparing a deck
    // that doesn't contain the already dealt-out cards
    let available_cards = CARDS
        .iter()
        .filter(|c| !player.contains(c))
        .filter(|c| !opponent.contains(c))
        .filter(|c| !board.contains(c));
    let mut deck = Deck::with_seed(available_cards, SEED);

    let mut pots_won = 0.0;
    for _ in 0..SAMPLE_SIZE {
        // Then, for each run we draw cards to complete the board
        deck.reset();
        let missing = 5 - board.len();
        let complete_board = board
            .iter()
            .chain(deck.deal(missing).unwrap().iter())
            .collect::<Hand>();

        // Evaluate the player's hand given the completed board
        let mut player_hand = *player;
        player_hand.extend(complete_board.iter());
        let player_rank = poker_rank(&player_hand);

        // Evaluate the opponent's hand
        let mut opponent_hand = *opponent;
        opponent_hand.extend(complete_board.iter());
        let opponent_rank = poker_rank(&opponent_hand);

        // And record the player's share of the pot for the run
        match player_rank.cmp(&opponent_rank) {
            Ordering::Greater => pots_won += 1.0,
            Ordering::Less => {}
            Ordering::Equal => pots_won += 0.5,
        };
    }

    pots_won / SAMPLE_SIZE as f64
}
```

## Performance

All of the following benchmarks were run on an AMD Ryzen 5 2600 and compiled
with `lto = "thin"`.

### Standard / Ace-to-Five Lowball / Deuce-to-Seven Lowball / Six-plus

![](https://github.com/dtrifuno/aya-poker/blob/main/benchmarks/standard.png?raw=true)

Standard poker, ace-to-five lowball, and six-or-better (short-deck) poker are the
fastest since they represent the ideal scenario for the lookup table approach:
they only require computing the `Hand` value, and then using it to perform a
single lookup.

Deuce-to-seven lowball is slightly slower for more than 5 cards due to
having to iterate over non-flush card combinations.

### Badugi / Baduci

![](https://github.com/dtrifuno/aya-poker/blob/main/benchmarks/badugi.png?raw=true)

Badugi / Baduci evaluation is much slower since at the moment AyaPoker only has
lookup tables for rank combinations with non-overlapping suits, and thus requires
a number of lookups for each such combination of cards.

### Omaha

![](https://github.com/dtrifuno/aya-poker/blob/main/benchmarks/omaha.png?raw=true)

AyaPoker does not include any lookup tables for Omaha since they are too large
to compute at compile time, but, due to the excellent performance of the lookup
tables for standard and ace-to-five lowball poker, is still able to achieve
decent performance for Omaha by iterating over all 5-card combinations.

## License

Licensed under any of:

- Apache License, Version 2.0, ([LICENSE-APACHE](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- zlib License ([LICENSE-ZLIB](https://raw.githubusercontent.com/dtrifuno/quickphf/main/LICENSE-ZLIB) or <https://opensource.org/license/zlib/>)

by your choice.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be multi-licensed as above, without any additional terms or conditions.
