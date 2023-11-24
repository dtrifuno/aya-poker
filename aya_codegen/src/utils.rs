use std::collections::{HashMap, HashSet};

use aya_base::constants::{FLUSH_RANKS, RANKS, RANK_COUNT};

pub fn num_cards(ranks: u64) -> usize {
    (0..RANK_COUNT)
        .map(|r| (ranks >> (4 * r)) & 0xf)
        .sum::<u64>() as usize
}

pub fn ranks_to_key(ranks: u64) -> u32 {
    (0..13)
        .map(|r| ((ranks >> (r * 4)) & 0xf) as u32 * RANKS[r])
        .sum()
}

pub fn ranks_to_flush_key(ranks: u64) -> u32 {
    (0..13)
        .map(|r| ((ranks >> (r * 4)) & 0xf) as u32 * FLUSH_RANKS[r])
        .sum()
}

pub fn generate_all_ranks(min_hand_size: usize, max_hand_size: usize) -> Vec<u64> {
    let mut result = Vec::new();

    let mut stack = vec![(0, 13)];
    while let Some((ranks, max_rank)) = stack.pop() {
        if num_cards(ranks) >= min_hand_size {
            result.push(ranks);
        }

        if num_cards(ranks) == max_hand_size {
            continue;
        }

        for r in 0..max_rank {
            let new_ranks = ranks + (1 << (4 * r));
            let r_rank_count = (new_ranks >> (4 * r)) & 0xf;

            if r_rank_count > 4 {
                continue;
            }

            stack.push((new_ranks, r + 1));
        }
    }
    result
}

pub fn get_biggest_straight(ranks: u64) -> usize {
    let rank_mask = (0x1_1111_1111_1111 & ranks)
        | ((0x2_2222_2222_2222 & ranks) >> 1)
        | ((0x4_4444_4444_4444 & ranks) >> 2);
    for i in (0..=9).rev() {
        if ((rank_mask >> (4 * i)) & 0x11111) == 0x11111 {
            return i + 4;
        }
    }

    if (rank_mask & 0x1_0000_0000_1111) == 0x1_0000_0000_1111 {
        3
    } else {
        0
    }
}

pub fn insert_ranks(
    lookup_table: &mut HashMap<u32, u16>,
    hand_value_classes: &Vec<Vec<u64>>,
    key_fn: impl Fn(u64) -> u32,
    last_hand_value: u16,
) -> u16 {
    let mut hand_value = last_hand_value;
    for value_class in hand_value_classes {
        hand_value += 1;
        for &ranks in value_class {
            let key = key_fn(ranks);
            lookup_table.insert(key, hand_value);
        }
    }
    hand_value
}

pub fn get_rank_count(ranks: u64, idx: usize) -> u64 {
    (ranks >> (4 * idx)) & 0xf
}

pub fn set_rank_count(ranks: u64, idx: usize, count: u64) -> u64 {
    let old_count = get_rank_count(ranks, idx);
    let mask = (old_count ^ count) << (4 * idx);
    ranks ^ mask
}

pub fn can_be_monochrome(ranks: u64) -> bool {
    (0..13).map(|r| (ranks >> (4 * r)) & 0xf).all(|r| r <= 1)
}

pub fn five_card_hands(starting_ranks: u64) -> Vec<u64> {
    let mut stack = vec![(starting_ranks, 0)];
    let mut result = Vec::new();

    while let Some((ranks, min_rank)) = stack.pop() {
        if num_cards(ranks) == 5 {
            result.push(ranks);
        }

        if num_cards(ranks) <= 5 {
            continue;
        }

        for r in min_rank..13 {
            let rank_count = get_rank_count(ranks, r);
            if rank_count == 0 {
                continue;
            }

            let new_ranks = set_rank_count(ranks, r, rank_count - 1);
            stack.push((new_ranks, r));
        }
    }

    result
}

pub fn insert_partial_hands<'a>(
    partial_hands: impl Iterator<Item = &'a u64>,
    result: &mut [Vec<u64>],
) {
    for &ranks in partial_hands {
        'outer: for hand_rank in result.iter_mut() {
            for hand_in_rank in hand_rank.iter() {
                if ranks & *hand_in_rank == ranks {
                    hand_rank.push(ranks);
                    break 'outer;
                }
            }
        }
    }
}

pub fn generate_lowball_ranks(
    starting_ranks: u64,
    valid_ranks: &[u64],
    min_hand_size: usize,
    max_rank_straight: usize,
) -> Vec<Vec<u64>> {
    let mut result = Vec::new();

    let mut stack = vec![(starting_ranks, 13)];
    let mut partial_hands = HashSet::new();

    while let Some((ranks, max_rank)) = stack.pop() {
        if num_cards(ranks) == 5 {
            result.push(vec![ranks]);
            continue;
        } else if num_cards(ranks) >= min_hand_size && num_cards(ranks) < 5 {
            partial_hands.insert(ranks);
        }

        for (i, r) in valid_ranks.iter().enumerate().take(max_rank) {
            let new_ranks = ranks + (1 << (4 * r));
            let r_rank_count = (new_ranks >> (4 * r)) & 0xf;

            if r_rank_count >= 2 {
                continue;
            }

            if get_biggest_straight(new_ranks) > max_rank_straight {
                continue;
            }

            stack.push((new_ranks, i + 1));
        }
    }

    insert_partial_hands(partial_hands.iter(), &mut result);

    result
}
