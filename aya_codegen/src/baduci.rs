use std::collections::HashMap;

use miniphf::CodeWriter;

use crate::{
    utils::{generate_all_ranks, insert_ranks, num_cards, ranks_to_key},
    HAND_CATEGORY_OFFSET,
};

pub struct BaduciLookup {
    ranks_lookup: HashMap<u32, u16>,
}

impl BaduciLookup {
    pub fn new() -> BaduciLookup {
        let mut table = BaduciLookup {
            ranks_lookup: HashMap::new(),
        };
        table.populate_tables();
        table
    }

    fn populate_tables(&mut self) {
        self.init_ranks_lookup();
        self.init_duplicated_ranks();
        assert_eq!(self.ranks_lookup.len(), 2379);
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
        // 1. One Card
        let hand_value = 0;
        let one_card_hands = self.generate_ranks(0, 1);
        insert_ranks(
            &mut self.ranks_lookup,
            &one_card_hands,
            ranks_to_key,
            hand_value,
        );

        // 2. Two Card
        let hand_value = HAND_CATEGORY_OFFSET;
        let two_card_hands = self.generate_ranks(0, 2);
        insert_ranks(
            &mut self.ranks_lookup,
            &two_card_hands,
            ranks_to_key,
            hand_value,
        );

        // 3. Three Card
        let hand_value = 2 * HAND_CATEGORY_OFFSET;
        let three_card_hands = self.generate_ranks(0, 3);
        insert_ranks(
            &mut self.ranks_lookup,
            &three_card_hands,
            ranks_to_key,
            hand_value,
        );

        // 4. Four Card
        let hand_value = 3 * HAND_CATEGORY_OFFSET;
        let four_card_hands = self.generate_ranks(0, 4);
        insert_ranks(
            &mut self.ranks_lookup,
            &four_card_hands,
            ranks_to_key,
            hand_value,
        );
    }

    fn init_duplicated_ranks(&mut self) {
        let all_ranks = generate_all_ranks(2, 4);
        for ranks in all_ranks {
            let deduped_ranks = Self::dedupe_ranks(ranks);
            if ranks != deduped_ranks {
                let deduped_key = ranks_to_key(deduped_ranks);
                let hand_value = *self.ranks_lookup.get(&deduped_key).unwrap();
                self.ranks_lookup.insert(ranks_to_key(ranks), hand_value);
            }
        }
    }

    fn generate_ranks(&self, starting_ranks: u64, hand_size: usize) -> Vec<Vec<u64>> {
        let mut result = Vec::new();

        let mut stack = vec![(starting_ranks, 13)];
        while let Some((ranks, max_rank)) = stack.pop() {
            if num_cards(ranks) == hand_size {
                result.push(vec![ranks]);
                continue;
            }

            for r in 0..max_rank {
                let new_ranks = ranks + (1 << (4 * r));
                let r_rank_count = (new_ranks >> (4 * r)) & 0xf;

                if r_rank_count > 1 {
                    continue;
                }

                stack.push((new_ranks, r + 1));
            }
        }

        result
    }

    fn dedupe_ranks(ranks: u64) -> u64 {
        (0..13)
            .map(|r| u64::min((ranks >> (4 * r)) & 0xf, 1))
            .enumerate()
            .fold(0, |acc, (i, r)| acc + (r << (4 * i)))
    }
}

impl Default for BaduciLookup {
    fn default() -> BaduciLookup {
        BaduciLookup::new()
    }
}
