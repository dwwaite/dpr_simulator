use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::{max, min};

use crate::{HitResult, RollBehaviour};

/// A representation of a collection of dice
#[derive(Debug)]
pub struct Dice {
    pub max: i32,
    min: i32,
    roll_behaviour: RollBehaviour,
    alt_value: i32,
    rng_element: StdRng,
}

// Implement PartialEq to avoid comparing the StdRng instance.
impl PartialEq for Dice {
    fn eq(&self, other: &Self) -> bool {
        (self.min, self.max, self.roll_behaviour, self.alt_value)
            == (other.min, other.max, other.roll_behaviour, other.alt_value)
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
    /// let my_die = Dice::new();
    /// ```
    #[allow(dead_code)]
    pub fn new() -> Dice {
        Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        }
    }

    /// Returns the result of a single dice roll
    ///
    /// # Examples
    /// ```
    /// let mut my_die = Dice::new();
    ///
    /// let result = my_die.make_roll();
    /// ```
    fn make_roll(&mut self) -> i32 {
        self.rng_element.random_range(self.min..self.max + 1)
    }

    /// Returns the result of a single dice roll using the alternate maximum
    ///
    /// # Examples
    /// ```
    /// let mut my_die = Dice::new();
    ///
    /// let result = my_die.make_roll();
    /// ```
    fn make_alt_roll(&mut self) -> i32 {
        self.rng_element.random_range(self.min..self.alt_value + 1)
    }

    /// Assess the value of the dice roll for an instance of application.
    ///
    /// Accepts an optional hit result for modulating the return result based
    /// on the RollBehaviour flag. If None is provided, the value is returned
    /// unmodified without consideration of a hit result. The main use case for
    /// this is for hit rolls, where as value modification is most likely to be
    /// required for damage rolls.
    ///
    /// # Examples
    /// ```
    /// let mut my_die = Dice::new();
    ///
    /// let result = my_die.evaluate_result(None);
    /// let result = my_die.evaluate_result(Some(&HitResult::Hit));
    /// ```
    pub fn evaluate_result(&mut self, hit_condition: Option<&HitResult>) -> i32 {
        // Code path for damage rolls, where hit result is considered
        if let Some(hit_result) = hit_condition {
            match (hit_result, &self.roll_behaviour) {
                (&HitResult::CriticalHit, &RollBehaviour::Fatal) => {
                    self.make_alt_roll() + self.make_alt_roll() + self.make_alt_roll()
                }
                (&HitResult::CriticalHit, &RollBehaviour::ExclusiveCrit) => self.make_roll(),
                (&HitResult::CriticalHit, _) => self.make_roll() + self.make_roll(),
                (&HitResult::Hit, &RollBehaviour::ExclusiveCrit) => 0,
                (&HitResult::Hit, _) => self.make_roll(),
                (_, _) => 0,
            }
        // Code paths for hit rolls, where hit result is not considered
        } else {
            match self.roll_behaviour {
                RollBehaviour::DoubleAdvantage => {
                    let roll_results = [self.make_roll(), self.make_roll(), self.make_roll()];
                    *roll_results.iter().max().unwrap()
                }
                RollBehaviour::Advantage => max(self.make_roll(), self.make_roll()),
                RollBehaviour::Disadvantage => min(self.make_roll(), self.make_roll()),
                _ => self.make_roll(),
            }
        }
    }
}

/// Builder pattern for DiceCollection struct
#[derive(Debug, PartialEq)]
pub struct DiceBuilder {
    min: i32,
    max: i32,
    roll_behaviour: RollBehaviour,
    alt_value: i32,
    rng_seed: Option<u64>,
}

impl DiceBuilder {
    /// Create a new builder with default 1d4 values
    ///
    /// # Examples
    /// ```
    /// let dice_builder = DiceBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_seed: None,
        }
    }

    /// Change the minimum roll value from the default of the DiceBuilder
    ///
    /// # Examples
    /// ```
    /// let dice_builder = DiceBuilder::new().set_roll_min(5);
    /// ```
    #[allow(dead_code)]
    pub fn set_roll_min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// Change the maximum roll value from the default of the DiceBuilder
    ///
    /// # Examples
    /// ```
    /// let dice_builder = DiceBuilder::new().set_roll_max(5);
    /// ```
    pub fn set_roll_max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// Change the roll behaviour (advantage, disadvantage, etc.) of the DiceBuilder
    ///
    /// # Examples
    /// ```
    /// let dice_builder = DiceBuilder::new().set_roll_behaviour(RollBehaviour::Advantage, None);
    /// ```
    pub fn set_roll_behaviour(
        mut self,
        roll_behaviour: RollBehaviour,
        alt_value: Option<i32>,
    ) -> Self {
        self.roll_behaviour = roll_behaviour;
        if let Some(value) = alt_value {
            self.alt_value = value;
        }
        self
    }

    /// Set the RNG for the roll seed to a specific value.
    ///
    /// # Examples
    /// ```
    /// let dice_builder = DiceBuilder::new().set_rng_seed(5);
    /// ```
    #[allow(dead_code)]
    pub fn set_rng_seed(mut self, seed: u64) -> Self {
        self.rng_seed = Some(seed);
        self
    }

    /// Return the configured Dice struct
    ///
    /// # Examples
    /// ```
    /// let dice_collection: Dice = DiceBuilder::new()
    ///     .roll_max(10)
    ///     .roll_behaviour(RollBehaviour::Disadvantage)
    ///     .build();
    /// ```
    pub fn build(self) -> Dice {
        let rng_init = match self.rng_seed {
            Some(u) => StdRng::seed_from_u64(u),
            None => StdRng::from_os_rng(),
        };

        Dice {
            min: self.min,
            max: self.max,
            roll_behaviour: self.roll_behaviour,
            alt_value: self.alt_value,
            rng_element: rng_init,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unpack_roll_vector(roll_capture: &Vec<i32>) -> (i32, i32) {
        let obs_min: i32 = *roll_capture.iter().min().unwrap();
        let obs_max: i32 = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }

    #[test]
    fn test_dice_constructor() {
        let exp_die = Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let obs_die = Dice::new();
        assert_eq!(exp_die, obs_die);
    }

    #[test]
    fn test_make_roll() {
        let mut my_die = Dice::new();

        let roll_results: Vec<i32> = (0..10_000).map(|_| my_die.make_roll()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!((1, 4), obs_results);
    }

    #[test]
    fn test_make_alt_roll() {
        let mut my_die = Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 10,
            rng_element: StdRng::from_os_rng(),
        };

        let roll_results: Vec<i32> = (0..10_000).map(|_| my_die.make_alt_roll()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!((1, 10), obs_results);
    }

    // region: Dice::evaluate_result() with HitResult tests

    #[test]
    fn test_evaluate_result_hit_fatal() {
        let mut my_die = Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Fatal,
            alt_value: 10,
            rng_element: StdRng::from_os_rng(),
        };

        // Test behaviour on a critical hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::CriticalHit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((3, 30), obs_results);

        // Test behaviour on a regular hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Hit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 4), obs_results);

        // Test behaviour on a miss
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Miss)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((0, 0), obs_results);
    }

    #[test]
    fn test_evaluate_result_hit_excl_crit() {
        let mut my_die = Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::ExclusiveCrit,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        // Test for result on critical hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::CriticalHit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 4), obs_results);

        // Test for result on regular hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Hit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((0, 0), obs_results);

        // Test for result on miss
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Miss)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((0, 0), obs_results);
    }

    #[test]
    fn test_evaluate_result_hit_standard() {
        let mut my_die = Dice {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        // Test for result on critical hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::CriticalHit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((2, 8), obs_results);

        // Test for result on regular hit
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Hit)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 4), obs_results);

        // Test for result on miss
        let roll_results: Vec<i32> = (0..10_000)
            .map(|_| my_die.evaluate_result(Some(&HitResult::Miss)))
            .collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((0, 0), obs_results);
    }

    // endregion:

    // region: Dice::evaluate_result() with no modifier tests

    #[test]
    fn test_evaluate_result_none_dbl_advantage() {
        // Test the Dice::roll() function by confirming that the average DoubleAdvantage
        // roll is lower than for Standard rolls.

        let mut adv_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Advantage,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let mut dbl_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::DoubleAdvantage,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let adv_total: i32 = (0..10_000)
            .map(|_| adv_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();
        let dbl_total: i32 = (0..10_000)
            .map(|_| dbl_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        assert!(dbl_total > adv_total);
    }

    #[test]
    fn test_evaluate_result_none_advantage() {
        // Test the Dice::evaluate_result() by function confirming that average Advantage
        // roll is lower than for Standard rolls.

        let mut std_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let mut adv_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Advantage,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let std_total: i32 = (0..10_000)
            .map(|_| std_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();
        let adv_total: i32 = (0..10_000)
            .map(|_| adv_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        assert!(adv_total > std_total);
    }

    #[test]
    fn test_evaluate_result_none_disadvantage() {
        // Test the Dice::evaluate_result() function by confirming that average Disadvantage
        // roll is lower than for Standard rolls.

        let mut std_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let mut dis_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Disadvantage,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let std_total: i32 = (0..10_000)
            .map(|_| std_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();
        let dis_total: i32 = (0..10_000)
            .map(|_| dis_die.evaluate_result(None))
            .collect::<Vec<_>>()
            .iter()
            .sum();

        assert!(dis_total < std_total);
    }

    #[test]
    fn test_evaluate_result_none_standard() {
        let mut std_die = Dice {
            min: 1,
            max: 20,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let roll_results: Vec<i32> = (0..10_000).map(|_| std_die.evaluate_result(None)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 20), obs_results);
    }

    // endregion:

    // region: DiceBuilder

    #[test]
    fn test_builder_constructor() {
        let exp_result = DiceBuilder {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_seed: None,
        };

        let obs_result = DiceBuilder::new();
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_set_roll_min() {
        let exp_result = DiceBuilder {
            min: 4,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_seed: None,
        };

        let obs_result = DiceBuilder::new().set_roll_min(4);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_set_roll_max() {
        let exp_result = DiceBuilder {
            min: 1,
            max: 10,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_seed: None,
        };

        let obs_result = DiceBuilder::new().set_roll_max(10);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_set_roll_behaviour() {
        let exp_result = DiceBuilder {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Advantage,
            alt_value: 0,
            rng_seed: None,
        };

        let obs_result = DiceBuilder::new().set_roll_behaviour(RollBehaviour::Advantage, None);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_set_roll_behaviour_fatal() {
        let exp_result = DiceBuilder {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Fatal,
            alt_value: 10,
            rng_seed: None,
        };

        let obs_result = DiceBuilder::new().set_roll_behaviour(RollBehaviour::Fatal, Some(10));
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_set_rng_seed() {
        let exp_result = DiceBuilder {
            min: 1,
            max: 4,
            roll_behaviour: RollBehaviour::Standard,
            alt_value: 0,
            rng_seed: Some(10),
        };

        let obs_result = DiceBuilder::new().set_rng_seed(10 as u64);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_builder_build() {
        // Test by setting each element to a value different from the default.
        // Indirectly test that the seed was set by rolling with a known output sequence
        // of five values for the given input seed.

        let exp_result = Dice {
            min: 2,
            max: 10,
            roll_behaviour: RollBehaviour::Advantage,
            alt_value: 0,
            rng_element: StdRng::from_os_rng(),
        };

        let mut obs_result = DiceBuilder::new()
            .set_roll_min(2)
            .set_roll_max(10)
            .set_roll_behaviour(RollBehaviour::Advantage, None)
            .set_rng_seed(20)
            .build();

        assert_eq!(exp_result, obs_result);

        for exp_value in [6, 9, 9, 8, 8] {
            assert_eq!(exp_value, obs_result.evaluate_result(None));
        }
    }
    // endregion:
}
