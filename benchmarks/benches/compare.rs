use aya_poker::{base::Hand, deck::FullDeck, poker_rank};

const SEED: u64 = 42;

fn main() {
    divan::main();
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn aya_poker<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let hand = cards.iter().collect::<Hand>();
            poker_rank(&hand);
        })
}

#[divan::bench(consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn aya_poker_unchecked<const N: usize>(bencher: divan::Bencher) {
    let mut deck = FullDeck::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            deck.reset();
            deck.deal(N).unwrap().to_vec()
        })
        .bench_local_refs(|cards| {
            let mut hand = Hand::new();
            for card in cards.iter() {
                hand.insert_unchecked(card);
            }
            poker_rank(&hand);
        })
}

#[divan::bench_group(name = "ckc-rs (0.1.14)", sample_count = 1000, sample_size = 1000)]
mod ckc_rs_benches {
    use ckc_rs::cards::HandRanker;

    use super::*;

    #[divan::bench(name = "5")]
    fn five_cards(bencher: divan::Bencher) {
        let mut rng = fastrand::Rng::with_seed(SEED);

        bencher
            .counter(divan::counter::ItemsCount::new(1u32))
            .with_inputs(|| {
                let cards = rng.choose_multiple(ckc_rs::deck::POKER_DECK.arr().iter().copied(), 5);
                let hand = ckc_rs::cards::five::Five::new(
                    cards[0], cards[1], cards[2], cards[3], cards[4],
                );
                hand
            })
            .bench_local_refs(|hand| hand.hand_rank())
    }

    #[divan::bench(name = "7")]
    fn seven_cards(bencher: divan::Bencher) {
        let mut rng = fastrand::Rng::with_seed(SEED);

        bencher
            .counter(divan::counter::ItemsCount::new(1u32))
            .with_inputs(|| {
                let cards = rng.choose_multiple(ckc_rs::deck::POKER_DECK.arr().iter().copied(), 7);
                let hand = ckc_rs::cards::seven::Seven::new(
                    ckc_rs::cards::two::Two::new(cards[5], cards[6]),
                    ckc_rs::cards::five::Five::new(
                        cards[0], cards[1], cards[2], cards[3], cards[4],
                    ),
                );
                hand
            })
            .bench_local_refs(|hand| hand.hand_rank())
    }
}

#[divan::bench(consts = [5, 7], name = "poker (0.5)", sample_count = 1000, sample_size = 1000)]
fn poker<const N: usize>(bencher: divan::Bencher) {
    let poker_eval = poker::Evaluator::new();
    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| poker::Card::generate_shuffled_deck().drain(..N).collect())
        .bench_local_refs(|hand: &mut Vec<_>| poker_eval.evaluate(hand).unwrap())
}

#[divan::bench_group(name = "rs_poker (2.0)", sample_count = 1000, sample_size = 1000)]
mod rs_poker_benches {
    use rs_poker::core::Rankable;

    #[divan::bench(name = "5")]
    fn five_cards(bencher: divan::Bencher) {
        bencher
            .counter(divan::counter::ItemsCount::new(1u32))
            .with_inputs(|| rs_poker::core::FlatDeck::default().sample(5))
            .bench_local_refs(|hand| hand.rank_five())
    }

    #[divan::bench(name = "7")]
    fn seven_cards(bencher: divan::Bencher) {
        bencher
            .counter(divan::counter::ItemsCount::new(1u32))
            .with_inputs(|| rs_poker::core::FlatDeck::default().sample(5))
            .bench_local_refs(|hand| hand.rank())
    }
}

#[divan::bench(name = "rust_poker (0.1.14)", consts = [5, 7], sample_count = 1000, sample_size = 1000)]
fn rust_poker<const N: usize>(bencher: divan::Bencher) {
    let mut rng = fastrand::Rng::with_seed(SEED);

    bencher
        .counter(divan::counter::ItemsCount::new(1u32))
        .with_inputs(|| {
            let mut deck = rust_poker::hand_evaluator::CARDS.to_vec();
            rng.shuffle(&mut deck);
            deck[..N].to_vec()
        })
        .bench_local_refs(|cards| {
            let mut hand = rust_poker::hand_evaluator::Hand::default();
            for card in cards.iter() {
                hand += *card;
            }
            rust_poker::hand_evaluator::evaluate(&hand)
        })
}
