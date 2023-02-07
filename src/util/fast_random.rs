pub struct FastRandom {
    seed: u64,
}

impl FastRandom {
    pub fn get(seed: u64) -> u64 {
        let a = seed ^ (seed << 21);
        let b = a ^ (a >> 35);
        b ^ (b << 4)
    }

    pub fn new(seed: u64) -> Self { Self { seed } }

    pub fn next(&mut self) -> u64 {
        self.seed = Self::get(self.seed);
        self.seed
    }

    pub fn next_less_than(&mut self, upper_bound: u64) -> u64 {
        self.next() % upper_bound
    }

    /// upper_bound is exclusive
    pub fn next_in_range(&mut self, min: u64, upper_bound: u64) -> u64 {
        min + self.next_less_than(upper_bound - min)
    }

    pub fn one_in(&mut self, upper_bound: u64) -> bool {
        self.next_less_than(upper_bound) == 0
    }
}