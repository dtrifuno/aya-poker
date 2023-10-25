use std::collections::HashMap;

// use itertools::Itertools;

use aya_base::constants::RANK_COUNT;
use miniphf::CodeWriter;

use crate::{
    utils::{
        five_card_hands, generate_all_ranks, generate_lowball_ranks, insert_ranks, ranks_to_key,
    },
    HAND_CATEGORY_OFFSET,
};

pub struct AceFiveLowballLookup {
    ranks_lookup: HashMap<u32, u16>,
}

impl AceFiveLowballLookup {
    pub fn new() -> AceFiveLowballLookup {
        let mut table = AceFiveLowballLookup {
            ranks_lookup: HashMap::new(),
        };
        table.populate_tables();
        table
    }

    pub fn populate_tables(&mut self) {
        self.init_ranks_lookup();
        self.init_six_and_seven_card_hands();
        assert_eq!(self.ranks_lookup.len(), 76155);
    }

    pub fn generate_phf(&self, c: f64, load_factor: f64) -> CodeWriter<u16> {
        let entries = self
            .ranks_lookup
            .iter()
            .map(|(k, v)| (*k as u64, *v))
            .collect::<Vec<_>>();
        miniphf::build_phf_map(entries, c, load_factor)
    }

    fn init_ranks_lookup(&mut self) {
        let rs: [u64; 13] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 12];

        // 1. Quads
        for (i, r) in rs.iter().enumerate() {
            let hand_value = 256 * (i as u16);
            let ranks = 4 << (4 * r);
            let quad_rs = self.generate_ranks(ranks);
            insert_ranks(&mut self.ranks_lookup, &quad_rs, ranks_to_key, hand_value);
        }

        // 2. Full houses
        for r1 in 0..RANK_COUNT {
            for r2 in 0..RANK_COUNT {
                if r1 != r2 {
                    let hand_value = HAND_CATEGORY_OFFSET + 256 * (r1 as u16) + 16 * (r2 as u16);
                    let ranks = (3 << (4 * rs[r1])) + (2 << (4 * rs[r2]));
                    let r1s_full_of_r2s = vec![vec![ranks]];
                    insert_ranks(
                        &mut self.ranks_lookup,
                        &r1s_full_of_r2s,
                        ranks_to_key,
                        hand_value,
                    );
                }
            }
        }

        // 3. Sets
        for (i, r) in rs.iter().enumerate() {
            let hand_value = 2 * HAND_CATEGORY_OFFSET + 256 * (i as u16);
            let sets_of_rs = self.generate_ranks(3 << (4 * r));
            insert_ranks(
                &mut self.ranks_lookup,
                &sets_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 4. Two pairs
        for r1 in 0..RANK_COUNT {
            for r2 in 0..r1 {
                let hand_value = 3 * HAND_CATEGORY_OFFSET + 256 * (r2 as u16) + 16 * (r1 as u16);
                let ranks = (2 << (4 * rs[r1])) + (2 << (4 * rs[r2]));
                let two_pairs_of_r1s_and_r2s = self.generate_ranks(ranks);
                insert_ranks(
                    &mut self.ranks_lookup,
                    &two_pairs_of_r1s_and_r2s,
                    ranks_to_key,
                    hand_value,
                );
            }
        }

        // 5. Pairs
        for (i, r) in rs.iter().enumerate() {
            let hand_value = 4 * HAND_CATEGORY_OFFSET + 256 * (i as u16);
            let pair_of_rs = self.generate_ranks(2 << (4 * r));
            insert_ranks(
                &mut self.ranks_lookup,
                &pair_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 6. High Cards
        let hand_value = 5 * HAND_CATEGORY_OFFSET;
        let high_cards = self.generate_ranks(0);
        insert_ranks(
            &mut self.ranks_lookup,
            &high_cards,
            ranks_to_key,
            hand_value,
        );
    }

    fn init_six_and_seven_card_hands(&mut self) {
        let overfilled_ranks: Vec<u64> = generate_all_ranks(6, 7);

        for ranks in overfilled_ranks {
            let mut max_ranks_ranking = 0;

            for five_card_ranks in five_card_hands(ranks) {
                let ranks_ranking = *self
                    .ranks_lookup
                    .get(&ranks_to_key(five_card_ranks))
                    .unwrap();
                max_ranks_ranking = max_ranks_ranking.max(ranks_ranking);

                self.ranks_lookup
                    .insert(ranks_to_key(ranks), max_ranks_ranking);
            }
        }
    }

    fn generate_ranks(&self, starting_ranks: u64) -> Vec<Vec<u64>> {
        generate_lowball_ranks(
            starting_ranks,
            &[12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            0,
            13,
        )
    }
}

impl Default for AceFiveLowballLookup {
    fn default() -> AceFiveLowballLookup {
        AceFiveLowballLookup::new()
    }
}
