pub struct OptimizedSampler {
    ratio: f64,
    // Use thread-local RNG for better performance
    rng: thread_local::ThreadLocal<rand::rngs::SmallRng>,
}

impl OptimizedSampler {
    pub fn should_sample(&self) -> bool {
        self.rng.get_or(|| {
            rand::rngs::SmallRng::from_entropy()
        }).gen::<f64>() < self.ratio
    }
}
