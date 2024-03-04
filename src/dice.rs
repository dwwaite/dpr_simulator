use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{max, min};

use crate::{DiceMode, Reroll};

#[derive(Debug, PartialEq)]
pub struct DiceCollection {
    n_dice: i32,
    roll_min: i32,
    roll_max: i32,
    roll_modifier: Reroll,
    dice_mode: DiceMode,
    alt_max: i32,
}

impl DiceCollection {
    pub fn new(n_dice: i32, roll_max: i32, roll_modifier: Reroll) -> DiceCollection {
        DiceCollection {
            n_dice: n_dice,
            roll_min: 1,
            roll_max: roll_max,
            roll_modifier: roll_modifier,
            dice_mode: DiceMode::Standard,
            alt_max: -1,
        }
    }

    fn make_roll(min: i32, max: i32, roll_element: &mut ThreadRng) -> i32 {
        // Roll the die element
        roll_element.gen_range(min..max + 1)
    }

    fn resolve_roll(roll_min: i32, roll_max: i32, roll_modifier: &Reroll, roll_element: &mut ThreadRng) -> i32 {
        // Resolve the outcome for the die rolls with regards to advantage mechanics.

        match roll_modifier {
            Reroll::Standard => DiceCollection::make_roll(roll_min, roll_max, roll_element),
            Reroll::Advantage => {
                max(
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                )
            },
            Reroll::DoubleAdvantage => {
                *vec![
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                ]
                .iter()
                .max()
                .unwrap()
            },
            Reroll::Disadvantage => {
                min(
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                    DiceCollection::make_roll(roll_min, roll_max, roll_element),
                )
            },
        }
    }

    pub fn increase_minimum(&mut self, roll_min: i32) {
        // Change the minimum roll size for die behaviour. Primarily used for unit testing.

        self.roll_min = roll_min;
    }

    pub fn set_fatal(&mut self, alt_max: i32) {
        // Change the die statistics for fatal die behaviour.

        self.alt_max = alt_max;
        self.dice_mode = DiceMode::Fatal;
    }

    pub fn roll(&self, roll_element: &mut ThreadRng) -> i32 {
        // Resolve the outcome for the regular die rolls with regards to advantage mechanics and dice behaviour.

        match self.dice_mode {
            // Currently there are no alternate paths for regular rolls.
            _ => {
                let running_total: i32 = (0..self.n_dice)
                    .map(|_| DiceCollection::resolve_roll(self.roll_min, self.roll_max, &self.roll_modifier, roll_element))
                    .sum();
                running_total
            }
        }
    }

    pub fn roll_critical(&self, roll_element: &mut ThreadRng) -> i32 {
        // Resolve the outcome for the regular die rolls with regards to advantage mechanics and dice behaviour.

        match self.dice_mode {
            DiceMode::Standard => self.roll(roll_element) + self.roll(roll_element),
            DiceMode::Fatal => {
                // Fatal rolls as double dice + 1, at an increased die size.
                let n_rolls = 2 * self.n_dice + 1;
                let running_total: i32 = (0..n_rolls)
                    .map(|_| DiceCollection::resolve_roll(self.roll_min, self.alt_max, &self.roll_modifier, roll_element))
                    .sum();
                running_total
            },
            _ => 0
        }
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    fn unpack_roll_vector(roll_capture: &Vec<i32>) -> (i32, i32) {
        let obs_min: i32 = *roll_capture.iter().min().unwrap();
        let obs_max: i32 = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }

    #[test]
    fn test_make_roll() {
        // Test the DiceCollection::make_roll() function, ensuring the minimum and maximum are captured.

        let mut roll_element = rand::thread_rng();
        let roll_results: Vec<i32> = (0..10000).map(|_| DiceCollection::make_roll(1, 10, &mut roll_element)).collect();

        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 10), obs_results);
    }

    #[test]
    fn test_resolve_roll_standard() {
        // Test the DiceCollection::resolve_roll() function for Standard dice roll mechanics.

        let mut roll_element = rand::thread_rng();
        let roll_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Standard, &mut roll_element)).collect();

        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 10), obs_results);
    }

    #[test]
    fn test_resolve_roll_advantage() {
        // Test the DiceCollection::resolve_roll() function for Advantage dice roll mechanics.
        // Essentially a confirmation that on average Advantage roll is higher than Standard rolls.

        let mut roll_element = rand::thread_rng();
        let std_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Standard, &mut roll_element)).collect();
        let adv_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Advantage, &mut roll_element)).collect();

        let std_total: i32 = std_results.iter().sum();
        let adv_total: i32 = adv_results.iter().sum();
        assert!(adv_total > std_total);
    }

    #[test]
    fn test_resolve_roll_doubleadvantage() {
        // Test the DiceCollection::resolve_roll() function for DoubleAdvantage dice roll mechanics.
        // Essentially a confirmation that on average DoubleAdvantage roll is higher than Advantage rolls.

        let mut roll_element = rand::thread_rng();
        let adv_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Advantage, &mut roll_element)).collect();
        let dbl_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::DoubleAdvantage, &mut roll_element)).collect();

        let adv_total: i32 = adv_results.iter().sum();
        let double_total: i32 = dbl_results.iter().sum();
        assert!(double_total > adv_total);
    }

    #[test]
    fn test_resolve_roll_disadvantage() {
        // Test the DiceCollection::resolve_roll() function for Disadvantage dice roll mechanics.
        // Essentially a confirmation that on average Disadvantage roll is lower than Standard rolls.

        let mut roll_element = rand::thread_rng();
        let std_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Advantage, &mut roll_element)).collect();
        let dis_results: Vec<i32> = (0..10000).map(|_| DiceCollection::resolve_roll(1, 10, &Reroll::Disadvantage, &mut roll_element)).collect();

        let std_total: i32 = std_results.iter().sum();
        let dis_total: i32 = dis_results.iter().sum();
        assert!(dis_total < std_total);
    }

    #[test]
    fn test_increase_minimum() {
        // Test the DiceCollection::set_fatal() function.

        let mut dc = DiceCollection::new(1, 6, Reroll::Standard);
        dc.increase_minimum(2);

        assert_eq!(2, dc.roll_min);
    }

    #[test]
    fn test_set_fatal() {
        // Test the DiceCollection::set_fatal() function.

        let mut dc = DiceCollection::new(1, 6, Reroll::Standard);
        dc.set_fatal(8);

        assert_eq!(dc.alt_max, 8);
        assert_eq!(dc.dice_mode, DiceMode::Fatal);
    }

    #[test]
    fn test_roll_standard() {
        // Test the DiceCollection::roll() function for default behaviour.
        // This is basically the instanced wrapper of the DiceCollection::resolve_roll() function
        //  so only testing on standard behaviour.

        let dc = DiceCollection::new(1, 10, Reroll::Standard);
        let mut roll_element = rand::thread_rng();

        let results: Vec<i32> = (0..10000).map(|_| dc.roll(&mut roll_element)).collect();
        let (obs_min, obs_max) = unpack_roll_vector(&results);
        assert_eq!(1, obs_min);
        assert_eq!(10, obs_max);
    }

    #[test]
    fn test_critical_roll_standard() {
        // Test the DiceCollection::roll_critical() function for standard behaviour.
        // Expected roll values are double the minimum and maximum per die rolled.

        let dc = DiceCollection::new(1, 10, Reroll::Standard);
        let mut roll_element = rand::thread_rng();

        let results: Vec<i32> = (0..10000).map(|_| dc.roll_critical(&mut roll_element)).collect();
        let (obs_min, obs_max) = unpack_roll_vector(&results);
        assert_eq!(2, obs_min);
        assert_eq!(20, obs_max);
    }

    #[test]
    fn test_critical_roll_fatal() {
        // Test the DiceCollection::roll_critical() function for fatal behaviour.
        // Expected roll values are 2x + 1 for minimum and maximum.

        let mut dc = DiceCollection::new(1, 10, Reroll::Standard);
        dc.set_fatal(12);

        let mut roll_element = rand::thread_rng();
        let results: Vec<i32> = (0..10000).map(|_| dc.roll_critical(&mut roll_element)).collect();
        let (obs_min, obs_max) = unpack_roll_vector(&results);
        assert_eq!(3, obs_min);
        assert_eq!(36, obs_max);
    }
}

//endregion
