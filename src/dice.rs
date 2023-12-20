use std::cmp::{max, min};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::Reroll;

const MIN_ROLL: i32 = 1;

#[derive(Debug, PartialEq)]
pub struct Die {
    roll_min: i32,
    roll_max: i32,
    roll_modifier: Reroll,
}

impl Die {
    pub fn new(roll_max: i32, roll_modifier: Reroll) -> Die {
        Die {
            roll_min: MIN_ROLL,
            roll_max: roll_max,
            roll_modifier: roll_modifier,
        }
    }

    pub fn roll(&self, roll_element: &mut ThreadRng) -> i32 {
        // Roll the outcome for the Die element.

        match self.roll_modifier {
            Reroll::Standard => roll_element.gen_range(self.roll_min..self.roll_max + 1),
            Reroll::Advantage => {
                max(
                    roll_element.gen_range(self.roll_min..self.roll_max + 1),
                    roll_element.gen_range(self.roll_min..self.roll_max + 1)
                )
            },
            Reroll::DoubleAdvantage => {
                *vec![
                    roll_element.gen_range(self.roll_min..self.roll_max + 1),
                    roll_element.gen_range(self.roll_min..self.roll_max + 1),
                    roll_element.gen_range(self.roll_min..self.roll_max + 1)
                ]
                .iter()
                .max()
                .unwrap()
            },
            Reroll::Disadvantage => {
                min(
                    roll_element.gen_range(self.roll_min..self.roll_max + 1),
                    roll_element.gen_range(self.roll_min..self.roll_max + 1)
                )
            },
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
    fn test_roll_standard() {
        let mut roll_element = rand::thread_rng();
        let my_die = Die::new(10, Reroll::Standard);

        let mut roll_results: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            roll_results.push(my_die.roll(&mut roll_element));
        }

        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);
        assert_eq!((1, 10), obs_results);
    }

    #[test]
    fn test_roll_advantage() {
        let mut roll_element = rand::thread_rng();
        let std_die = Die::new(10, Reroll::Standard);
        let adv_die = Die::new(10, Reroll::Advantage);

        let mut std_results: Vec<i32> = Vec::new();
        let mut adv_results: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            std_results.push(std_die.roll(&mut roll_element));
            adv_results.push(adv_die.roll(&mut roll_element));
        }

        // Take the sum of both vectors. Expectation is that the die with
        //  advantage will sum to greater than standard die.
        let std_total: i32 = std_results.iter().sum();
        let adv_total: i32 = adv_results.iter().sum();
        assert!(adv_total > std_total);
    }

    #[test]
    fn test_roll_disadvantage() {
        let mut roll_element = rand::thread_rng();
        let std_single = Die::new(10, Reroll::Standard);
        let adv_die = Die::new(10, Reroll::Disadvantage);

        let mut std_results: Vec<i32> = Vec::new();
        let mut adv_results: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            std_results.push(std_single.roll(&mut roll_element));
            adv_results.push(adv_die.roll(&mut roll_element));
        }

        // Take the sum of both vectors. Expectation is that the die with
        //  disadvantage will sum to less than standard die.
        let std_total: i32 = std_results.iter().sum();
        let adv_total: i32 = adv_results.iter().sum();
        assert!(adv_total < std_total);
    }

    #[test]
    fn test_roll_doubleadvantage() {
        let mut roll_element = rand::thread_rng();
        let adv_die = Die::new(10, Reroll::Advantage);
        let double_die = Die::new(10, Reroll::DoubleAdvantage);

        let mut adv_results: Vec<i32> = Vec::new();
        let mut double_results: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            adv_results.push(adv_die.roll(&mut roll_element));
            double_results.push(double_die.roll(&mut roll_element));
        }

        // Take the sum of both vectors. Expectation is that the die with
        //  double-advantage will sum to greater than the advantage die.
        let adv_total: i32 = adv_results.iter().sum();
        let double_total: i32 = double_results.iter().sum();
        assert!(double_total > adv_total);
    }
}

//endregion
