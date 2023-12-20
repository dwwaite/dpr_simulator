use rand::rngs::ThreadRng;
use regex::Regex;

use crate::{dice::Die, Reroll};

#[derive(Debug, PartialEq)]
pub struct DiceContext {
    dice: Vec<Die>,
    pub static_modifier: i32,
}

impl DiceContext {
    pub fn new(dice: Vec<Die>, static_modifier: i32) -> DiceContext {
        DiceContext {
            dice: dice,
            static_modifier: static_modifier,
        }
    }

    fn parse_die_elements(notation: &str) -> (i32, i32) {
        // Return a tuple representing the number of die and size, extracted from
        // the notation string.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_die = Regex::new(r"\d+d\d+").unwrap();

        let die_elements: (i32, i32) = match regex_die
            .find(notation) {
                Some(ref x) => {

                    let tokens: Vec<&str> = x.as_str().split("d").collect();
                    let n_die = tokens[0].parse().unwrap();
                    let s_die = tokens[1].parse().unwrap();

                    (n_die, s_die)
                },
                None => (0, 0)
            };

        die_elements
    }

    fn parse_reroll_elements(notation: &str) -> Reroll {
        // Set the reroll flag - standard roll, roll with advantage, or roll
        // with disadvantage.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_reroll = Regex::new(r"AA|A|D").unwrap();
        let value_capture: &str = match regex_reroll
            .find(notation) {
                Some(x) => x.as_str(),
                None => ""
            };

        match value_capture {
            "AA" => Reroll::DoubleAdvantage,
            "A" => Reroll::Advantage,
            "D" => Reroll::Disadvantage,
            _ => Reroll::Standard
        }
    }

    fn parse_static_elements(notation: &str) -> i32 {
        // Return the static modifier from notation string, as either a positive
        // or negative integer.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_static = Regex::new(r"(\+\d+)|(-\d+)").unwrap();

        let value_capture: i32 = regex_static
            .find_iter(notation)
            .map(|m| {
                // Can safely unwrap these parse calls, as the regex above ensures the values
                //  are valid numeric digits.
                m.as_str().parse::<i32>().unwrap()
            })
            .sum();

        value_capture
    }

    pub fn parse_dice_string(notation: &str) -> DiceContext {
        // Parse the notation string into individual die notations, and the static modifier.

        // Parse the individual die in the notation string.
        let die_vector: Vec<Die> = notation
            .split(",")
            .map(|n| {

                let (n_die, die_size) = DiceContext::parse_die_elements(n);

                let mut die_vector: Vec<Die> = Vec::new();
                for _ in 0..n_die {
                    die_vector.push(Die::new(
                        die_size,
                        DiceContext::parse_reroll_elements(n)));
                }
                die_vector
            })
            .into_iter()
            .flatten()
            .collect();

        let static_modifier: i32 = DiceContext::parse_static_elements(notation);

        DiceContext::new(die_vector, static_modifier)
    }

    pub fn roll(&self, roll_element: &mut ThreadRng) -> i32 {
        // Roll the outcome for the DiceContext element.

        self.dice
            .iter()
            .map(|d| d.roll(roll_element))
            .into_iter()
            .sum()
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_die_elements() {
        let exp_vector: (i32, i32) = (1, 8);
        let obs_vector: (i32, i32) = DiceContext::parse_die_elements("1d8+5");

        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_die_elements_none() {
        let exp_vector: (i32, i32) = (0, 0);
        let obs_vector: (i32, i32) = DiceContext::parse_die_elements("+5");

        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_static_elements_single_pos() {
        let obs_result: i32 = DiceContext::parse_static_elements("1d8+5");
        assert_eq!(5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_single_neg() {
        let obs_result: i32 = DiceContext::parse_static_elements("1d8-5");
        assert_eq!(-5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_multiple() {
        let obs_result: i32 = DiceContext::parse_static_elements("1d8+5-3");
        assert_eq!(2, obs_result);
    }

    #[test]
    fn test_parse_reroll_elements_single_a() {
        for s in vec!["1d4A", "A1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Advantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_single_aa() {
        for s in vec!["1d4AA", "AA1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::DoubleAdvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_single_d() {
        for s in vec!["1d4D", "D1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Disadvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_mixed_a() {
        for s in vec!["A1d4D", "AD1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Advantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_mixed_d() {
        for s in vec!["D1d4A", "DA1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Disadvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_default() {
        let obs_result: Reroll = DiceContext::parse_reroll_elements("1d4");
        assert_eq!(Reroll::Standard, obs_result);
    }

    #[test]
    fn test_parse_dice_string() {
        let exp_context = DiceContext::new(
            vec![Die::new(4, Reroll::Standard), Die::new(6, Reroll::Disadvantage), Die::new(6, Reroll::Disadvantage)],
            7
        );

        let obs_context: DiceContext = DiceContext::parse_dice_string("1d4,D2d6+7");
        assert_eq!(exp_context, obs_context);
    }

    #[test]
    fn test_roll() {
        let mut roll_element = rand::thread_rng();
        let context = DiceContext::new(
            vec![Die::new(4, Reroll::Standard), Die::new(4, Reroll::Standard)],
            20
        );

        // Total of the roll must be at least 1 + 1 but less than the static modifier
        let obs_result = context.roll(&mut roll_element);
        assert!(obs_result >= 2);
        assert!(obs_result < 20);
    }
}

//endregion
