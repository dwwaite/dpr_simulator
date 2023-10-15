use crate::dice::Die;
use rand::rngs::ThreadRng;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct DamageElement {
    pub to_hit: i32,
    die_elements: Vec<Die>,
    static_element: i32,
}

impl DamageElement {
    pub fn new(to_hit: i32, die_elements: Vec<Die>, static_element: i32) -> DamageElement {
        DamageElement {
            to_hit: to_hit,
            die_elements: die_elements,
            static_element: static_element,
        }
    }

    fn parse_die_elements(notation: &str) -> Vec<Die> {
        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_die = Regex::new(r"\d+d\d+").unwrap();

        let die_elements: Vec<Die> = regex_die
            .find_iter(notation)
            .map(|m| {
                // Can safely unwrap these parse calls, as the regex above ensures the values
                //  are valid numeric digits.
                let tokens: Vec<&str> = m.as_str().split("d").collect();
                let n_die = tokens[0].parse().unwrap();
                let s_die = tokens[1].parse().unwrap();

                Die::new(n_die, s_die)
            })
            .collect();

        die_elements
    }

    fn parse_static_elements(notation: &str) -> i32 {
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

    pub fn from_notation_string(to_hit: i32, notation: &str) -> DamageElement {
        /* Uses regex to decompose the following elements from an input string:
                XdY
                +N
                -N
            Extracts as many of each as found, to allow for flexible inputs like a paladin
                using a 1d6 weapon with Improved Divine Strike (1d6 + 1d8 + mod).
        */

        let die_elements: Vec<Die> = DamageElement::parse_die_elements(notation);
        let static_modifier: i32 = DamageElement::parse_static_elements(notation);

        DamageElement::new(to_hit, die_elements, static_modifier)
    }

    pub fn roll_damage(&self, roll_element: &mut ThreadRng) -> (i32, i32) {
        // Roll the damage for the DamageElement

        let mut dmg_roll: i32 = 0;

        for die in &self.die_elements {
            dmg_roll += die.roll(roll_element)
        }

        (dmg_roll, self.static_element)
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
    fn test_parse_die_elements_single() {
        let exp_vector: Vec<Die> = vec![Die::new(1, 8)];
        let obs_vector: Vec<Die> = DamageElement::parse_die_elements("1d8+5");

        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_die_elements_multiple() {
        let exp_vector: Vec<Die> = vec![Die::new(1, 6), Die::new(1, 8)];
        let obs_vector: Vec<Die> = DamageElement::parse_die_elements("1d6,1d8+5");

        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_static_elements_single_pos() {
        let obs_result: i32 = DamageElement::parse_static_elements("1d8+5");
        assert_eq!(5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_single_neg() {
        let obs_result: i32 = DamageElement::parse_static_elements("1d8-5");
        assert_eq!(-5, obs_result);
    }

    #[test]
    fn test_parse_static_elements_multiple() {
        let obs_result: i32 = DamageElement::parse_static_elements("1d8+5-3");
        assert_eq!(2, obs_result);
    }

    #[test]
    fn test_from_notation_string() {
        // Test the extraction for die notation

        let exp_element = DamageElement::new(0, vec![Die::new(1, 4), Die::new(2, 6)], 5);
        let obs_element = DamageElement::from_notation_string(0, "1d4,2d6+5");

        assert_eq!(exp_element, obs_element);
    }

    #[test]
    fn test_from_notation_string_empty() {
        // Test the extraction for die notation on a fail case.

        let obs_result: DamageElement = DamageElement::from_notation_string(0, "");
        assert_eq!(DamageElement::from_notation_string(0, ""), obs_result);
    }

    #[test]
    fn test_create_empty() {
        let obs_element = DamageElement::from_notation_string(0, "");

        assert_eq!(obs_element.die_elements, Vec::<Die>::new());
        assert_eq!(obs_element.static_element, 0);
    }

    #[test]
    fn test_roll_damage_single() {
        // Tests the roll function over a single-die hit.

        // Rolling 1d4+1 - should range between 2 and 5
        let my_element: DamageElement = DamageElement::new(0, vec![Die::new(1, 4)], 1);
        let mut roll_element = rand::thread_rng();

        let mut die_capture: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            let (die_result, mod_result) = my_element.roll_damage(&mut roll_element);
            die_capture.push(die_result + mod_result);
        }

        let (obs_min, obs_max) = unpack_roll_vector(&die_capture);
        assert_eq!(obs_min, 2);
        assert_eq!(obs_max, 5);
    }

    #[test]
    fn test_roll_damage_multi() {
        // Tests the roll function over a multi-die hit.

        // Rolling 1d4 + 1d6 + 1, should range between 3 and 11
        let my_element: DamageElement =
            DamageElement::new(0, vec![Die::new(1, 4), Die::new(1, 6)], 1);
        let mut roll_element = rand::thread_rng();

        let mut roll_capture: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            let (die_result, mod_result) = my_element.roll_damage(&mut roll_element);
            roll_capture.push(die_result + mod_result);
        }

        let (obs_min, obs_max) = unpack_roll_vector(&roll_capture);
        assert_eq!(3, obs_min);
        assert_eq!(11, obs_max);
    }
}

//endregion
