use std::collections::{HashMap, HashSet};

use aya_base::constants::RANK_COUNT;
use miniphf::CodeWriter;

use crate::{
    utils::{
        can_be_monochrome, five_card_hands, generate_all_ranks, get_biggest_straight,
        insert_partial_hands, insert_ranks, num_cards, ranks_to_flush_key, ranks_to_key,
    },
    HAND_CATEGORY_OFFSET,
};

pub struct SixPlusPokerLookup {
    flush_lookup: HashMap<u32, u16>,
    ranks_lookup: HashMap<u32, u16>,
}

impl SixPlusPokerLookup {
    pub fn new() -> SixPlusPokerLookup {
        let mut result = SixPlusPokerLookup {
            flush_lookup: HashMap::new(),
            ranks_lookup: HashMap::new(),
        };
        result.populate_tables();
        result
    }

    fn populate_tables(&mut self) {
        self.init_flush_lookup();
        self.init_ranks_lookup();
        self.init_six_and_seven_card_hands();

        assert_eq!(self.ranks_lookup.len(), 10945);
        assert_eq!(self.flush_lookup.len(), 246);
    }

    pub fn generate_ranks_phf(&self, c: f64, load_factor: f64) -> CodeWriter<u16> {
        let entries = self
            .ranks_lookup
            .iter()
            .map(|(k, v)| (*k as u64, *v))
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
        // 1. High Cards
        let mut hand_value = 0;
        let high_cards = self.generate_ranks(0, 0);
        insert_ranks(
            &mut self.ranks_lookup,
            &high_cards,
            ranks_to_key,
            hand_value,
        );

        // 2. Pairs
        for r in 4..RANK_COUNT {
            hand_value = HAND_CATEGORY_OFFSET + 256 * (r as u16);
            let pair_of_rs = self.generate_ranks(2 << (4 * r), 2);
            insert_ranks(
                &mut self.ranks_lookup,
                &pair_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 3. Two pairs
        for r1 in 4..RANK_COUNT {
            for r2 in 4..r1 {
                hand_value = 2 * HAND_CATEGORY_OFFSET + 256 * (r1 as u16) + 16 * (r2 as u16);
                let two_pairs_of_r1s_and_r2s =
                    self.generate_ranks((2 << (4 * r1)) + (2 << (4 * r2)), 4);
                insert_ranks(
                    &mut self.ranks_lookup,
                    &two_pairs_of_r1s_and_r2s,
                    ranks_to_key,
                    hand_value,
                );
            }
        }

        // 4. Sets
        for r in 4..RANK_COUNT {
            hand_value = 3 * HAND_CATEGORY_OFFSET + 256 * (r as u16);
            let sets_of_rs = self.generate_ranks(3 << (4 * r), 3);
            insert_ranks(
                &mut self.ranks_lookup,
                &sets_of_rs,
                ranks_to_key,
                hand_value,
            );
        }

        // 5. Straights
        hand_value = 4 * HAND_CATEGORY_OFFSET;
        let mut straights = vec![vec![0x1_0000_1111_0000]];
        straights.extend((8..RANK_COUNT).map(|r| vec![0x11111u64 << (4 * (r - 4))]));
        insert_ranks(&mut self.ranks_lookup, &straights, ranks_to_key, hand_value);

        // 6. Full houses
        for r1 in 4..RANK_COUNT {
            for r2 in 4..RANK_COUNT {
                if r1 != r2 {
                    hand_value = 5 * HAND_CATEGORY_OFFSET + 256 * (r1 as u16) + 16 * (r2 as u16);
                    let ranks = (3 << (4 * r1)) + (2 << (4 * r2));
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

        // 7. Quads
        for r in 4..RANK_COUNT {
            hand_value = 7 * HAND_CATEGORY_OFFSET + 256 * (r as u16);
            let ranks = 4 << (4 * r);
            let quad_rs = self.generate_ranks(ranks, 4);
            insert_ranks(&mut self.ranks_lookup, &quad_rs, ranks_to_key, hand_value);
        }
    }

    fn init_flush_lookup(&mut self) {
        // 1. Flushes
        let mut hand_value = 6 * HAND_CATEGORY_OFFSET;
        let high_cards = self.generate_ranks(0, 5);
        insert_ranks(
            &mut self.flush_lookup,
            &high_cards,
            ranks_to_flush_key,
            hand_value,
        );

        // 2. Straight flushes
        hand_value = 8 * HAND_CATEGORY_OFFSET;
        let mut straights = vec![vec![0x1_0000_1111_0000]];
        straights.extend((8..(RANK_COUNT - 1)).map(|r| vec![0x11111u64 << (4 * (r - 4))]));
        insert_ranks(
            &mut self.flush_lookup,
            &straights,
            ranks_to_flush_key,
            hand_value,
        );

        // 3. Royal flush
        hand_value = 9 * HAND_CATEGORY_OFFSET;
        let royal_flush = vec![vec![0x1_1111_0000_0000]];
        insert_ranks(
            &mut self.flush_lookup,
            &royal_flush,
            ranks_to_flush_key,
            hand_value,
        );
    }

    fn init_six_and_seven_card_hands(&mut self) {
        let overfull_ranks = generate_all_ranks(6, 7)
            .into_iter()
            .filter(|x| x.trailing_zeros() >= 16)
            .collect::<Vec<_>>();

        for ranks in overfull_ranks {
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
        let valid_ranks = &[12, 11, 10, 9, 8, 7, 6, 5, 4];
        let mut result = Vec::new();
        let mut stack = vec![(starting_ranks, 0)];
        let mut partial_hands = HashSet::new();

        while let Some((ranks, min_rank)) = stack.pop() {
            if num_cards(ranks) == 5 {
                result.push(vec![ranks]);
                continue;
            } else if num_cards(ranks) >= min_hand_size {
                partial_hands.insert(ranks);
            }

            for (i, r) in valid_ranks.iter().enumerate().skip(min_rank) {
                let new_ranks = ranks + (1 << (4 * r));
                let r_rank_count = (new_ranks >> (4 * r)) & 0xf;

                let is_six_plus_wheel = new_ranks & 0x1_0000_1111_0000 == 0x1_0000_1111_0000;
                if r_rank_count >= 2 || get_biggest_straight(new_ranks) > 0 || is_six_plus_wheel {
                    continue;
                }

                stack.push((new_ranks, i + 1));
            }
        }

        insert_partial_hands(partial_hands.iter(), &mut result);

        result
    }
}

impl Default for SixPlusPokerLookup {
    fn default() -> SixPlusPokerLookup {
        SixPlusPokerLookup::new()
    }
}
