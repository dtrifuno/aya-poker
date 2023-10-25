use std::{env, fs::File, io::BufWriter, io::Write, path::Path};

use aya_codegen::{
    AceFiveLowballLookup, BaduciLookup, BadugiLookup, DeuceSevenLowballLookup, PokerLookup,
    SixPlusPokerLookup,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Standard poker
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("holdem.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let builder = PokerLookup::new();
    let phf1 = builder.generate_ranks_phf(3.0, 0.95);
    let phf2 = builder.generate_flush_phf(2.5, 0.95);

    writeln!(
        &mut file,
        "static HOLDEM_RANKS_PHF: crate::MiniPhf = {};\n",
        phf1
    )
    .unwrap();
    writeln!(
        &mut file,
        "static HOLDEM_FLUSH_PHF: crate::MiniPhf = {};\n",
        phf2
    )
    .unwrap();

    // Ace-to-five lowball poker
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("ace_five.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());
    let builder = AceFiveLowballLookup::new();
    let phf = builder.generate_phf(2.8, 0.96);
    writeln!(
        &mut file,
        "static ACE_FIVE_RANKS_PHF: crate::MiniPhf = {};\n",
        phf
    )
    .unwrap();

    // Baduci
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("baduci.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());
    let builder = BaduciLookup::new();
    let phf = builder.generate_phf(1.8, 0.99);
    writeln!(&mut file, "static BADUCI_PHF: crate::MiniPhf = {};\n", phf).unwrap();

    // Badugi
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("badugi.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());
    let builder = BadugiLookup::new();
    let phf = builder.generate_phf(1.8, 0.99);
    writeln!(&mut file, "static BADUGI_PHF: crate::MiniPhf = {};\n", phf).unwrap();

    // Deuce-to-seven lowball poker
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("deuce_seven.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let builder = DeuceSevenLowballLookup::new();
    let phf1 = builder.generate_ranks_phf(3.0, 0.95);
    let phf2 = builder.generate_flush_phf(2.5, 0.95);

    writeln!(
        &mut file,
        "static DEUCE_SEVEN_RANKS_PHF: crate::MiniPhf = {};\n",
        phf1
    )
    .unwrap();
    writeln!(
        &mut file,
        "static DEUCE_SEVEN_FLUSH_PHF: crate::MiniPhf = {};\n",
        phf2
    )
    .unwrap();

    // Short deck poker
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("short_deck.rs");
    let mut file = BufWriter::new(File::create(path).unwrap());

    let builder = SixPlusPokerLookup::new();
    let ranks_phf = builder.generate_ranks_phf(2.0, 0.99);
    let flush_phf = builder.generate_flush_phf(2.0, 0.99);

    writeln!(
        &mut file,
        "static SIX_PLUS_RANKS_PHF: crate::MiniPhf = {};\n",
        ranks_phf
    )
    .unwrap();
    writeln!(
        &mut file,
        "static SIX_PLUS_FLUSH_PHF: crate::MiniPhf = {};\n",
        flush_phf
    )
    .unwrap();
}
