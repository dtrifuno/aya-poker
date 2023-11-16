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
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            ace_five_rank(&hand);
        })
}

#[divan::bench(consts = [4, 5], sample_count = 1000, sample_size = 1000)]
fn baduci<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            baduci_rank(&hand);
        })
}

#[divan::bench(consts = [4, 5], sample_count = 1000, sample_size = 1000)]
fn badugi<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            badugi_rank(&hand);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn deuce_seven<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            deuce_seven_rank(&hand);
        })
}

#[divan::bench(consts = [2, 4, 5], sample_count = 1000, sample_size = 1000)]
fn omaha<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            let hole_cards = deck.deal(N).unwrap().to_vec();
            let community_cards = deck.deal(5).unwrap().to_vec();
            (hole_cards, community_cards)
        })
        .bench_local_refs(|(hole_cards, community_cards)| {
            let hole = hole_cards.iter().collect();
            let board = community_cards.iter().collect();
            omaha_rank(&hole, &board);
        })
}

#[divan::bench(consts = [2, 4, 5], sample_count = 1000, sample_size = 1000)]
fn omaha_lo<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            let hole_cards = deck.deal(N).unwrap().to_vec();
            let community_cards = deck.deal(5).unwrap().to_vec();
            (hole_cards, community_cards)
        })
        .bench_local_refs(|(hole_cards, community_cards)| {
            let hole = hole_cards.iter().collect();
            let board = community_cards.iter().collect();
            omaha_lo_rank(&hole, &board);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn short_deck<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            ace_five_rank(&hand);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn standard<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect();
            poker_rank(&hand);
        })
}
