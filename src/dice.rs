use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RollBehaviour {
    Standard,
    KeepHighFromX { extra_rolls: i32, },
    KeepLowFromX { extra_rolls: i32, },
}

/// A representation of a collection of dice
#[derive(Debug)]
pub struct Dice {
    pub sides: i32,
    pub roll_behaviour: RollBehaviour,
    rng: StdRng,
}

// Implement PartialEq to avoid comparing the StdRng instance.
impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        (self.sides == other.sides) && (self.roll_behaviour == other.roll_behaviour)
    }
}

impl Dice {
    /// Creates a new Dice representation.
    ///
    /// Sets default values for all internal variables, using a 1d4 as the base.
    /// The expectation is that the DiceBuilder is used to toggle values before
    /// obtaining a completed Dice struct.
    ///
    /// # Examples
    ///
    /// ```
    /// let my_die = Dice::new(4, None);
    ///
    /// let my_die = Dice::new(4, Some(5));
    /// ```
    pub fn new(
        sides: i32,
        rng_seed: Option<u64>,
    ) -> Dice {
        Dice {
            sides: sides,
            roll_behaviour: RollBehaviour::Standard,
            rng: match rng_seed {
                Some(u) => StdRng::seed_from_u64(u),
                None => StdRng::from_os_rng(),
            }
        }
    }

    pub fn with_roll_behaviour(mut self, roll_behaviour: RollBehaviour) -> Self {
        self.roll_behaviour = roll_behaviour;
        self
    }

    /// Returns the result of a single dice roll using a specified die size.
    ///
    /// # Examples
    /// ```
    /// let mut my_die = Dice::new(4, None);
    ///
    /// let result = my_die.make_roll(self.sides);
    /// let result = my_die.make_roll(6);
    /// ```
    pub fn roll(&mut self, n_sides: i32) -> i32 {
        match self.roll_behaviour {
            RollBehaviour::Standard => self.rng.random_range(1..=n_sides),
            RollBehaviour::KeepHighFromX { extra_rolls } => {
                let total_rolls = extra_rolls + 1;
                (0..total_rolls)
                    .map(|_| self.rng.random_range(1..=n_sides))
                    .max()
                    .unwrap()
            },
            RollBehaviour::KeepLowFromX { extra_rolls } => {
                let total_rolls = extra_rolls + 1;
                (0..total_rolls)
                    .map(|_| self.rng.random_range(1..=n_sides))
                    .min()
                    .unwrap()
            },
            
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError::NotUnicode;

use super::*;

    fn unpack_roll_vector(roll_capture: &Vec<i32>) -> (i32, i32) {
        let obs_min: i32 = *roll_capture.iter().min().unwrap();
        let obs_max: i32 = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }

    // region: Dice constructor and initialisation tests

    #[test]
    fn test_dice_constructor_set_roll() {
        let exp_die = Dice {
            sides: 1,
            roll_behaviour: RollBehaviour::Standard,
            rng: StdRng::from_os_rng(),
        };

        let obs_die = Dice::new(1, None);
        assert_eq!(exp_die, obs_die);
    }

    #[test]
    fn test_dice_constructor_no_roll() {
        let exp_die = Dice {
            sides: 1,
            roll_behaviour: RollBehaviour::Standard,
            rng: StdRng::from_os_rng(),
        };

        let obs_die = Dice::new(1, None);
        assert_eq!(exp_die, obs_die);
    }

    #[test]
    fn test_with_roll_behaviour() {
        let mut my_die = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        assert_eq!(RollBehaviour::KeepHighFromX { extra_rolls: 2 }, my_die.roll_behaviour);
    }

    #[test]
    fn test_dice_eq() {
        let dice_1 = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        let dice_2 = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        assert_eq!(dice_1, dice_2);
    }

    #[test]
    fn test_dice_ne_sides() {
        let dice_1 = Dice::new(5, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        let dice_2 = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        assert_ne!(dice_1, dice_2);
    }

    #[test]
    fn test_dice_ne_behaviour() {
        let dice_1 = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 });

        let dice_2 = Dice::new(4, None)
            .with_roll_behaviour(RollBehaviour::Standard);

        assert_ne!(dice_1, dice_2);
    }

    // endregion:

    // region: Dice::roll tests

    #[test]
    fn test_roll_deterministic_with_seed() {
        // Same seed should produce same result
        let mut dice1 = Dice::new(1_000, Some(77777u64));
        let mut dice2 = Dice::new(1_000, Some(77777u64));

        let obs_result1 = dice1.roll(dice1.sides);
        let obs_result2 = dice2.roll(dice2.sides);

        assert_eq!(obs_result1, obs_result2);
    }

    #[test]
    fn test_roll_standard() {
        let mut my_die = Dice::new(4, None);

        let roll_results: Vec<i32> = (0..10_000).map(|_| my_die.roll(my_die.sides)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!((1, 4), obs_results);
    }

    #[test]
    fn test_roll_standard_other() {
        let mut my_die = Dice::new(4, None);

        let roll_results: Vec<i32> = (0..10_000).map(|_| my_die.roll(10)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!((1, 10), obs_results);
    }

        #[test]
    fn test_roll_keep_high() {
        // Test by confirming that the average roll with KeepHighFromX behaviour is greater than that
        // of standard.  
        let mut std_dice = Dice::new(100, None);
        let mut kh_die = Dice::new(100, None)
            .with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 10 });

        let std_total: i32 = (0..10_000)
            .map(|_| std_dice.roll(std_dice.sides))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        let kh_total: i32 = (0..10_000)
            .map(|_| kh_die.roll(kh_die.sides))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        assert!(kh_total > std_total);
    }

    #[test]
    fn test_roll_keep_low() {
        // Test by confirming that the average roll with KeepHighFromX behaviour is greater than that
        // of standard.  
        let mut std_dice = Dice::new(100, None);
        let mut kl_die = Dice::new(100, None)
            .with_roll_behaviour(RollBehaviour::KeepLowFromX { extra_rolls: 10 });

        let std_total: i32 = (0..10_000)
            .map(|_| std_dice.roll(std_dice.sides))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        let kl_total: i32 = (0..10_000)
            .map(|_| kl_die.roll(kl_die.sides))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        assert!(kl_total < std_total);
    }

    // endregion:

}
