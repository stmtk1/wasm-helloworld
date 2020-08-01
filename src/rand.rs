pub struct Rand {
    seed: u64,
}

impl Rand {
    pub fn new(seed: f64) -> Rand {
        Rand {
            seed: seed as u64,
        }
    }

    fn next_seed(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }

    pub fn random(&mut self) -> f64 {
        (self.next_seed() as f64) / (std::u64::MAX as f64)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    #[test]
    fn random_test() {
        let mut rnd = super::Rand::new(1.0);
        for _ in 0..100 {
            // writeln!(&mut std::io::stderr(), "{}", rnd.random()).unwrap();
            let x = rnd.random();
            assert!(0.0 <= x && x <= 1.0);
        }
    }
}
