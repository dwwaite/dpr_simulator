#[derive(Debug, PartialEq)]
pub struct MutationSeed {
    init_seed: Option<u64>,
}

impl MutationSeed {
    pub fn new(init_seed: Option<u64>) -> MutationSeed {
        MutationSeed { init_seed }
    }

    pub fn next_seed(&mut self) -> Option<u64> {
        self.init_seed.as_mut().map(|seed| {
            *seed += 1;
            *seed
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let exp_seed = MutationSeed { init_seed: Some(1) };
        let obs_seed = MutationSeed::new(Some(1));

        assert_eq!(exp_seed, obs_seed);
    }

    #[test]
    fn test_next_seed_single() {
        let mut m_seed = MutationSeed::new(Some(1));
        assert_eq!(Some(2), m_seed.next_seed());
    }

    #[test]
    fn test_next_seed_multiple() {
        let mut m_seed = MutationSeed::new(Some(1));

        for exp_seed in 2..=5 {
            assert_eq!(Some(exp_seed), m_seed.next_seed());
        }
    }

    #[test]
    fn test_next_seed_none() {
        let mut m_seed = MutationSeed::new(None);
        assert!(m_seed.next_seed().is_none());
    }
}
