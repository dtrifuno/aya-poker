use aya_codegen::{
    AceFiveLowballLookup, BaduciLookup, BadugiLookup, DeuceSevenLowballLookup, PokerLookup,
    SixPlusPokerLookup,
};

fn main() {
    divan::main();
}

#[divan::bench()]
fn standard_poker() {
    let builder = PokerLookup::new();
    builder.generate_ranks_phf(3.0, 0.95);
    builder.generate_flush_phf(2.5, 0.95);
}

#[divan::bench()]
fn ace_five() {
    let builder = AceFiveLowballLookup::new();
    builder.generate_phf(2.8, 0.96);
}

#[divan::bench()]
fn baduci() {
    let builder = BaduciLookup::new();
    builder.generate_phf(1.8, 0.99);
}

#[divan::bench()]
fn badugi() {
    let builder = BadugiLookup::new();
    builder.generate_phf(1.8, 0.99);
}

#[divan::bench()]
fn deuce_seven() {
    let builder = DeuceSevenLowballLookup::new();
    builder.generate_ranks_phf(3.0, 0.95);
    builder.generate_flush_phf(2.5, 0.95);
}

#[divan::bench()]
fn six_plus() {
    let builder = SixPlusPokerLookup::new();
    builder.generate_ranks_phf(2.0, 0.99);
    builder.generate_flush_phf(2.0, 0.99);
}
