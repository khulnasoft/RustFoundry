pub(crate) struct TraceSampler {
    ratio: f64,
    rng: rand::ThreadRng,
}

impl TraceSampler {
    pub fn should_sample(&mut self) -> bool {
        self.rng.gen::<f64>() < self.ratio
    }
}
