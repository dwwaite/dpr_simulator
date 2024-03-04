use rand::rngs::ThreadRng;
use regex::Regex;

use crate::{dice::DiceCollection, Reroll};

const ROLL_MIN: i32 = 1;

#[derive(Debug, PartialEq)]
pub struct DiceContext {
    dice: Vec<DiceCollection>,
    pub static_modifier: i32,
}

impl DiceContext {
    pub fn new(dice: Vec<DiceCollection>, static_modifier: i32) -> DiceContext {
        DiceContext {
            dice: dice,
            static_modifier: static_modifier,
        }
    }

    fn parse_regular_die(notation: &str) -> Option<(i32, i32)> {
        // Return a tuple representing the number of die and size, extracted from
        // the notation string.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_regular = Regex::new(r"\d+d\d+").unwrap();

        match regex_regular.find(notation) {
    
            Some(ref x) => {
                let tokens: Vec<&str> = x.as_str().split("d").collect();
                let n_die = tokens[0].parse().unwrap();
                let s_die = tokens[1].parse().unwrap();
    
                Some((n_die, s_die))
            }
            None => None,
        }
    }

    fn parse_fatal_die(notation: &str) -> Option<(i32, i32, i32)> {
        // Return a tuple representing the number of die, standard size, and fatal size
        //  extracted from the notation string.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_fatal = Regex::new(r"(?<f>\d+f\d+\~\d+)").unwrap();
        let regex_split = Regex::new(r"f|\~").unwrap();

        match regex_fatal.find(notation) {

            Some(ref x) => {
                let string_capture: &str = x.as_str();
                let tokens: Vec<i32> = regex_split.split(string_capture)
                    .map(|x| x.parse().unwrap())
                    .collect();
    
                Some((tokens[0], tokens[1], tokens[2]))
            }
            None => None,
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

    fn parse_reroll_elements(notation: &str) -> Reroll {
        // Set the reroll flag - standard roll, roll with advantage, or roll
        // with disadvantage.

        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_reroll = Regex::new(r"AA|A|D").unwrap();
        let value_capture: &str = match regex_reroll.find(notation) {
            Some(x) => x.as_str(),
            None => "",
        };

        match value_capture {
            "AA" => Reroll::DoubleAdvantage,
            "A" => Reroll::Advantage,
            "D" => Reroll::Disadvantage,
            _ => Reroll::Standard,
        }
    }

    pub fn parse_dice_string(notation: &str) -> DiceContext {
        // Parse the notation string into individual die notations, and the static modifier.

        let mut dice_vector: Vec<DiceCollection> = Vec::new();

        // Extract the recycling elements
        let static_modifier: i32 = DiceContext::parse_static_elements(notation);

        // Parse the die from the notation string.
        for n in notation.split(",") {

            if let Some((n_die, die_size)) = DiceContext::parse_regular_die(n) {
                // If a standard (1d4) notation is found, store a base die collection.

                let reroll_modifier = DiceContext::parse_reroll_elements(n);
                dice_vector.push(DiceCollection::new(n_die, die_size, reroll_modifier));

            } else if let Some((n_die, die_size, fatal_size)) = DiceContext::parse_fatal_die(n) {
                // If a fatal (1f4~6) notation is found, store as a die with the Fatal behaviour.

                let reroll_modifier = DiceContext::parse_reroll_elements(n);
                let mut f_dice = DiceCollection::new(n_die, die_size, reroll_modifier);
                f_dice.set_fatal(fatal_size);

                dice_vector.push(f_dice);
            }
        }

        DiceContext::new(dice_vector, static_modifier)
    }

    pub fn roll(&self, roll_element: &mut ThreadRng) -> i32 {
        // Roll the outcome for the DiceContext element.

        self.dice
            .iter()
            .map(|d| d.roll(roll_element))
            .sum()
    }

    pub fn roll_critical(&self, roll_element: &mut ThreadRng) -> i32 {
        // Roll the outcome for the DiceContext element.

        self.dice
            .iter()
            .map(|d| d.roll_critical(roll_element))
            .sum()
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_die_elements() {
        // Test the DiceContext.parse_regular_die() on a standard notation.

        let exp_vector = Some((1, 8));
        let obs_vector = DiceContext::parse_regular_die("1d8+5");

        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_die_elements_none() {
        // Test the DiceContext.parse_regular_die() on a string with no die notation.

        let obs_vector = DiceContext::parse_regular_die("+5");
        assert_eq!(None, obs_vector);
    }

    #[test]
    fn test_parse_static_elements_single_pos() {
    // Test the DiceContext.parse_static_elements() function for a positive element.

        let obs_result: i32 = DiceContext::parse_static_elements("1d8+5");
        assert_eq!(5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_single_neg() {
        // Test the DiceContext.parse_static_elements() function for a negative element.

        let obs_result: i32 = DiceContext::parse_static_elements("1d8-5");
        assert_eq!(-5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_multiple() {
        // Test the DiceContext.parse_static_elements() function for mixed elements.

        let obs_result: i32 = DiceContext::parse_static_elements("1d8+5-3");
        assert_eq!(2, obs_result);
    }

    #[test]
    fn test_parse_reroll_elements_single_a() {
        // Test the DiceContext.parse_reroll_elements() function for Advantage strings.

        for s in vec!["1d4A", "A1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Advantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_single_aa() {
        // Test the DiceContext.parse_reroll_elements() function for DoubleAdvantage strings.

        for s in vec!["1d4AA", "AA1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::DoubleAdvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_single_d() {
        // Test the DiceContext.parse_reroll_elements() function for Disadvantage strings.

        for s in vec!["1d4D", "D1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Disadvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_mixed_a() {
        // Test the DiceContext.parse_reroll_elements() function for mixed strings with Advantage first.

        for s in vec!["A1d4D", "AD1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Advantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_mixed_d() {
        // Test the DiceContext.parse_reroll_elements() function for mixed strings with Disadvantage first.

        for s in vec!["D1d4A", "DA1d4"] {
            let obs_result: Reroll = DiceContext::parse_reroll_elements(s);
            assert_eq!(Reroll::Disadvantage, obs_result);
        }
    }

    #[test]
    fn test_parse_reroll_elements_default() {
        // Test the DiceContext.parse_reroll_elements() function.

        let obs_result: Reroll = DiceContext::parse_reroll_elements("1d4");
        assert_eq!(Reroll::Standard, obs_result);
    }

    #[test]
    fn test_parse_dice_string_standard() {
        // Test the DiceContext.parse_dice_string() function for regular die only, covering
        //  all advantage states.

        let exp_context = DiceContext::new(
            vec![
                DiceCollection::new(1, 4, Reroll::Standard),
                DiceCollection::new(2, 6, Reroll::Disadvantage),
                DiceCollection::new(3, 6, Reroll::Advantage),
                DiceCollection::new(4, 6, Reroll::DoubleAdvantage),
            ],
            7,
        );

        let obs_context: DiceContext = DiceContext::parse_dice_string("1d4,D2d6,3d6A,4d6AA+7");
        assert_eq!(exp_context, obs_context);
    }

    #[test]
    fn test_parse_dice_string_fatal() {
        // Test the DiceContext.parse_dice_string() function for a regular die and fatal die, for only
        //  standard reroll states.

        let mut f_die = DiceCollection::new(2, 6, Reroll::Standard);
        f_die.set_fatal(8);

        let exp_context = DiceContext::new(
            vec![
                DiceCollection::new(1, 4, Reroll::Standard),
                f_die
            ],
            7,
        );

        let obs_context: DiceContext = DiceContext::parse_dice_string("1d4,2f6~8+7");
        assert_eq!(exp_context, obs_context);
    }

    #[test]
    fn test_roll() {
        // Test the DiceContext.roll() function. Behaviour is only for regular die, as other behaviours
        //  are tested within the DiceCollection struct.

        let mut roll_element = rand::thread_rng();
        let context = DiceContext::new(
            vec![
                DiceCollection::new(1, 4, Reroll::Standard),
                DiceCollection::new(2, 6, Reroll::Standard),
            ],
            20,
        );

        // Total of the roll must be greater than (3 * die, with minimum size each), but less than the static modifier
        let obs_result = context.roll(&mut roll_element);
        assert!(obs_result >= 3);
        assert!(obs_result < 20);
    }

    #[test]
    fn test_roll_critical() {
        // Test the DiceContext.roll_critical() function. Behaviour is only for regular die, as other behaviours
        //  are tested within the DiceCollection struct.

        let mut roll_element = rand::thread_rng();
        let context = DiceContext::new(
            vec![
                DiceCollection::new(1, 4, Reroll::Standard),
                DiceCollection::new(2, 6, Reroll::Standard),
            ],
            100,
        );

        // Total of the roll must be greater than (2 * 3 * die, with minimum size each), but less than the static modifier
        let obs_result = context.roll_critical(&mut roll_element);
        assert!(obs_result >= 6);
        assert!(obs_result < 100);
    }
}

//endregion
