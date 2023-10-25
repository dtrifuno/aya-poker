use quickdiv::DivisorU64;
use std::fmt;

/// Sentinel value indicating slot is not occupied.
const EMPTY: u32 = u32::MAX;

pub fn build_phf_map<V>(entries: Vec<(u64, V)>, c: f64, alpha: f64) -> CodeWriter<V> {
    let lg = (usize::BITS - entries.len().leading_zeros() - 1) as f64;
    let n = entries.len() as f64;
    let buckets_len = (c * n / lg).ceil() as u64;
    let codomain_len = {
        let candidate = (n / alpha).ceil() as u64;
        if candidate % 2 == 0 {
            candidate + 1
        } else {
            candidate
        }
    };

    let keys = entries.iter().map(|(k, _)| k);
    let phf = generate_phf(keys, codomain_len, buckets_len);
    CodeWriter { phf, entries }
}

pub struct CodeWriter<V> {
    phf: Phf,
    entries: Vec<(u64, V)>,
}

impl<V: fmt::Debug> fmt::Display for CodeWriter<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "crate::MiniPhf::new(")?;

        write!(f, "&[")?;
        for &idx in &self.phf.map {
            if idx == EMPTY {
                write!(f, "0,")?;
                continue;
            }
            write!(f, "{:?},", self.entries[idx as usize].1)?;
        }
        write!(f, "    ],")?;

        write!(f, "&[")?;
        for &pilot in &self.phf.pilots_table {
            write!(f, "{:?},", pilot)?;
        }
        write!(f, "])")
    }
}

fn hash_pilot_value(pilot_value: u64) -> u32 {
    /// Multiplicative constant from `fxhash`.
    const K: u64 = 0x517cc1b727220a95;
    pilot_value.wrapping_mul(K) as u32
}

/// Parameters for a PTHash perfect hash function.
#[derive(Debug)]
struct Phf {
    pilots_table: Vec<u32>,
    map: Vec<u32>,
}

/// Generate a perfect hash function using PTHash for the given collection of keys.
fn generate_phf<'a>(keys: impl Iterator<Item = &'a u64>, n_prime: u64, m: u64) -> Phf {
    let buckets_len = DivisorU64::new(m);
    let codomain_len = DivisorU64::new(n_prime);

    // We begin by hashing the entries, assigning them to buckets, and checking for collisions.
    struct HashedEntry {
        idx: usize,
        hash: u64,
        bucket: usize,
    }

    let mut hashed_entries: Vec<_> = keys
        .enumerate()
        .map(|(idx, &key)| {
            let hash = key;
            let bucket = (hash % buckets_len) as usize;

            HashedEntry { idx, hash, bucket }
        })
        .collect();

    hashed_entries.sort_unstable_by_key(|e| (e.bucket, e.hash));

    //
    struct BucketData {
        idx: usize,
        start_idx: usize,
        size: usize,
    }

    let mut buckets = Vec::with_capacity(buckets_len.get() as usize);

    let mut start_idx = 0;
    for idx in 0..buckets_len.get() as usize {
        let size = hashed_entries[start_idx..]
            .iter()
            .take_while(|entry| entry.bucket == idx)
            .count();

        buckets.push(BucketData {
            idx,
            start_idx,
            size,
        });
        start_idx += size;
    }

    buckets.sort_unstable_by(|b1, b2| b1.size.cmp(&b2.size).reverse());

    let mut pilots_table = vec![0; buckets_len.get() as usize];
    // Using a sentinel value instead of an Option here allows us to avoid an expensive
    // reallocation. This is fine since the compiler cannot handle a static map with more than
    // a few million entries anyway.
    let mut map = vec![EMPTY; codomain_len.get() as usize];

    let mut values_to_add = Vec::new();
    for bucket in buckets {
        let bucket_start = bucket.start_idx;
        let bucket_end = bucket_start + bucket.size;
        let bucket_entries = &hashed_entries[bucket_start..bucket_end];

        'pilots: for pilot in 0u64.. {
            values_to_add.clear();
            let pilot_hash = hash_pilot_value(pilot);

            // Check for collisions with items from previous buckets.
            for entry in bucket_entries.iter() {
                let destination = (entry.hash ^ pilot_hash as u64) % codomain_len;

                if map[destination as usize] != EMPTY {
                    continue 'pilots;
                }

                values_to_add.push((entry.idx, destination));
            }

            // Check for collisions within this bucket.
            values_to_add.sort_unstable_by_key(|k| k.1);
            for window in values_to_add.as_slice().windows(2) {
                if window[0].1 == window[1].1 {
                    continue 'pilots;
                }
            }

            for &(idx, destination) in &values_to_add {
                map[destination as usize] = idx as u32;
            }
            pilots_table[bucket.idx] = pilot_hash;
            break;
        }
    }

    Phf { pilots_table, map }
}
