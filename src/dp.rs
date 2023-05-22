use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut seen_lengths = HashMap::new();
        count_possibilities(
            &mut seen_lengths,
            0,
            low as usize,
            high as usize,
            zero as usize,
            one as usize,
        ) as i32
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = HashMap::with_capacity(n as usize);
        Self::climb_cache(n, &mut cache)
    }

    fn climb_cache(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        match cache.get(&n) {
            Some(&count) => count,
            None => match n {
                1 => 1,
                2 => 2,
                n => Self::climb_cache(n - 1, cache) + Self::climb_cache(n - 2, cache),
            },
        }
    }
}

fn count_possibilities(
    seen: &mut HashMap<usize, usize>,
    len: usize,
    low: usize,
    high: usize,
    zero: usize,
    one: usize,
) -> usize {
    if len > high {
        return 0;
    }
    if let Some(count) = seen.get(&len) {
        return *count;
    }
    let additional = if len >= low { 1 } else { 0 };
    let zero_count = count_possibilities(seen, len + zero, low, high, zero, one) % MOD;
    let one_count = count_possibilities(seen, len + one, low, high, zero, one) % MOD;

    seen.insert(len + zero, zero_count);
    seen.insert(len + one, one_count);
    (zero_count + one_count + additional) % MOD
}
