use std::collections::HashMap;

use aya_base::constants::RANK_COUNT;
use miniphf::CodeWriter;

use crate::{
    utils::{
        can_be_monochrome, five_card_hands, generate_all_ranks, generate_lowball_ranks,
        insert_ranks, ranks_to_flush_key, ranks_to_key,
    },
    HAND_CATEGORY_OFFSET,
};

pub struct DeuceSevenLowballLookup {
    flush_lookup: HashMap<u32, u16>,
    ranks_lookup: HashMap<u32, u16>,
}

impl DeuceSevenLowballLookup {
    pub fn new() -> DeuceSevenLowballLookup {
        let mut table = DeuceSevenLowballLookup {
            flush_lookup: HashMap::new(),
            ranks_lookup: HashMap::new(),
        };
        table.populate_tables();
        table
    }

    fn populate_tables(&mut self) {
        self.init_flush_lookup();
        self.init_ranks_lookup();
        self.init_six_and_seven_card_hands();

        assert_eq!(self.ranks_lookup.len(), 76155);
        assert_eq!(self.flush_lookup.len(), 4719);
    }

    pub fn generate_ranks_phf(&self, c: f64, load_factor: f64) -> CodeWriter<u16> {
        let entries = self
            .ranks_lookup
            .iter()
            .map(|(&k, &v)| (k as u64, v))
            .collect::<Vec<_>>();
        miniphf::build_phf_map(entries, c, load_factor)
    }

    pub fn generate_flush_phf(&self, c: f64, load_factor: f64) -> CodeWriter<u16> {
        let entries = self
            .flush_lookup
            .iter()
            .map(|(k, v)| (*k as u64, *v))
            .collect::<Vec<_>>();
        miniphf::build_phf_map(entries, c, load_factor)
    }

    fn init_ranks_lookup(&mut self) {
        let rs: [u64; 13] = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

        // 3. Quads
        for (i, r) in rs.iter().enumerate() {
            let hand_value = 2 * HAND_CATEGORY_OFFSET + 256 * (i as u16);
            let ranks = 4 << (4 * r);
            let quad_rs = self.generate_ranks(ranks, 4);
            insert_ranks(&mut self.ranks_lookup, &quad_rs, ranks_to_key, hand_value);
        }

        // 4. Full houses
        for r1 in 0..RANK_COUNT {
            for r2 in 0..RANK_COUNT {
                if r1 != r2 {
                    let hand_value =
                        3 * HAND_CATEGORY_OFFSET + 256 * (r1 as u16) + 16 * (r2 as u16);
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

        // 6. Straights
        let mut hand_value = 5 * HAND_CATEGORY_OFFSET;
        let mut straights = (4..RANK_COUNT)
            .map(|r| vec![0x11111 << (4 * (12 - r))])
            .collect::<Vec<Vec<_>>>();
        let wheel = vec![0x1_0000_0000_1111];
        straights.push(wheel);
        insert_ranks(&mut self.ranks_lookup, &straights, ranks_to_key, hand_value);

        // 7. Sets
        for (i, r) in rs.iter().enumerate() {
            hand_value = 6 * HAND_CATEGORY_OFFSET + 256 * (i as u16);
            let sets_of_rs = self.generate_ranks(3 << (4 * r), 3);
            insert_ranks(
                &mut self.ranks_lookup,
                &sets_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 8. Two pairs
        for r1 in 0..RANK_COUNT {
            for r2 in 0..r1 {
                hand_value = 7 * HAND_CATEGORY_OFFSET + 256 * (r2 as u16) + 16 * (r1 as u16);
                let two_pairs_of_r1s_and_r2s =
                    self.generate_ranks((2 << (4 * rs[r1])) + (2 << (4 * rs[r2])), 4);
                insert_ranks(
                    &mut self.ranks_lookup,
                    &two_pairs_of_r1s_and_r2s,
                    ranks_to_key,
                    hand_value,
                );
            }
        }

        // 9. Pairs
        for (i, r) in rs.iter().enumerate() {
            hand_value = 8 * HAND_CATEGORY_OFFSET + 256 * (i as u16);
            let pair_of_rs = self.generate_ranks(2 << (4 * r), 2);
            insert_ranks(
                &mut self.ranks_lookup,
                &pair_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 10. High Cards
        let hand_value = 9 * HAND_CATEGORY_OFFSET;
        let high_cards = self.generate_ranks(0, 0);
        insert_ranks(
            &mut self.ranks_lookup,
            &high_cards,
            ranks_to_key,
            hand_value,
        );
    }

    fn init_flush_lookup(&mut self) {
        // 1. Royal flush
        let hand_value = 0;
        let royal_flush = vec![vec![0x11111 << (4 * 8)]];
        insert_ranks(
            &mut self.flush_lookup,
            &royal_flush,
            ranks_to_flush_key,
            hand_value,
        );

        // 2. Straight flushes
        let mut hand_value = HAND_CATEGORY_OFFSET + 1;

        for r in 5..RANK_COUNT {
            let r_high_straight = vec![vec![0x11111 << (4 * (12 - r))]];
            hand_value = insert_ranks(
                &mut self.flush_lookup,
                &r_high_straight,
                ranks_to_flush_key,
                hand_value,
            );
        }

        let wheel = vec![vec![0x1_0000_0000_1111]];
        insert_ranks(
            &mut self.flush_lookup,
            &wheel,
            ranks_to_flush_key,
            hand_value,
        );

        // 5. Flushes
        let hand_value = 4 * HAND_CATEGORY_OFFSET;
        let high_cards = self.generate_ranks(0, 5);
        insert_ranks(
            &mut self.flush_lookup,
            &high_cards,
            ranks_to_flush_key,
            hand_value,
        );
    }

    fn init_six_and_seven_card_hands(&mut self) {
        let overfilled_ranks = generate_all_ranks(6, 7);

        for ranks in overfilled_ranks {
            let mut max_ranks_ranking = 0;
            let mut max_flush_ranking = 0;

            for five_card_ranks in five_card_hands(ranks) {
                let ranks_ranking = *self
                    .ranks_lookup
                    .get(&ranks_to_key(five_card_ranks))
                    .unwrap();
                max_ranks_ranking = max_ranks_ranking.max(ranks_ranking);

                if can_be_monochrome(ranks) {
                    let flush_ranking = *self
                        .flush_lookup
                        .get(&ranks_to_flush_key(five_card_ranks))
                        .unwrap();
                    max_flush_ranking = max_flush_ranking.max(flush_ranking);
                }
            }

            self.ranks_lookup
                .insert(ranks_to_key(ranks), max_ranks_ranking);
            if max_flush_ranking > 0 {
                self.flush_lookup
                    .insert(ranks_to_flush_key(ranks), max_flush_ranking);
            }
        }
    }

    fn generate_ranks(&self, starting_ranks: u64, min_hand_size: usize) -> Vec<Vec<u64>> {
        generate_lowball_ranks(
            starting_ranks,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            min_hand_size,
            0,
        )
    }
}

impl Default for DeuceSevenLowballLookup {
    fn default() -> DeuceSevenLowballLookup {
        DeuceSevenLowballLookup::new()
    }
}
