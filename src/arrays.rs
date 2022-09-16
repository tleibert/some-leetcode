use rand::Rng;

pub struct Solution {
    prefix: Vec<i32>,
    random: rand::rngs::ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let prefix = w
            .into_iter()
            .scan(0, |sum, item| {
                *sum += item;
                Some(*sum)
            })
            .collect();
        Self {
            prefix,
            random: rand::thread_rng(),
        }
    }

    pub fn pick_index(&mut self) -> i32 {
        let num = self.random.gen_range(0..*self.prefix.last().unwrap());

        match self.prefix.binary_search(&num) {
            Ok(idx) => idx as i32,
            Err(idx) => idx as i32,
        }
    }
}
