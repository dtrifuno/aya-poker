use core::cell::RefCell;

use aya_poker::base::*;
use aya_poker::deck::*;
use aya_poker::{
    ace_five_rank, baduci_rank, badugi_rank, deuce_seven_rank, omaha_lo_rank, omaha_rank,
    poker_rank,
};

const SEED: u64 = 42;

fn main() {
    divan::main();
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn ace_five<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(FullDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            ace_five_rank(&hand);
        })
}

#[divan::bench(consts = [4, 5], sample_count = 1000, sample_size = 1000)]
fn baduci<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(FullDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            baduci_rank(&hand);
        })
}

#[divan::bench(consts = [4, 5], sample_count = 1000, sample_size = 1000)]
fn badugi<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(FullDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            badugi_rank(&hand);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn deuce_seven<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(FullDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            deuce_seven_rank(&hand);
        })
}

#[divan::bench(consts = [2, 4, 5], sample_count = 1000, sample_size = 1000)]
fn omaha<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(ShortDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            let hole_cards = deck.borrow_mut().deal(N).unwrap().to_vec();
            let community_cards = deck.borrow_mut().deal(5).unwrap().to_vec();
            (hole_cards, community_cards)
        })
        .bench_local_refs(|(hole_cards, community_cards)| {
            let hole = hole_cards.iter().collect::<Hand>();
            let board = community_cards.iter().collect::<Hand>();
            omaha_rank(&hole, &board);
        })
}

#[divan::bench(consts = [2, 4, 5], sample_count = 1000, sample_size = 1000)]
fn omaha_lo<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(ShortDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            let hole_cards = deck.borrow_mut().deal(N).unwrap().to_vec();
            let community_cards = deck.borrow_mut().deal(5).unwrap().to_vec();
            (hole_cards, community_cards)
        })
        .bench_local_refs(|(hole_cards, community_cards)| {
            let hole = hole_cards.iter().collect::<Hand>();
            let board = community_cards.iter().collect::<Hand>();
            omaha_lo_rank(&hole, &board);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn short_deck<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(ShortDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            ace_five_rank(&hand);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn standard<const N: usize>(bencher: divan::Bencher) {
    let deck = RefCell::new(FullDeck::with_seed(SEED));

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.borrow_mut().reset();
            deck.borrow_mut().deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            poker_rank(&hand);
        })
}
