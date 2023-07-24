//use rand::Rng;
//use rand::rngs::ThreadRng;
use std::error::Error;
use regex::Regex;
use simple_error::bail;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Die {
    roll_number: i32,
    roll_min: i32,
    roll_max: i32,
}

impl Die {

    pub fn new(roll_number: i32, roll_min: i32, roll_max: i32) -> Die {
        Die {
            roll_number: roll_number,
            roll_min: roll_min,
            roll_max: roll_max,
        }
    }
}

#[derive(Debug)]
pub struct DamageElement {
    die_elements: Vec<Die>,
    static_elements: Vec<i32>,
}

impl DamageElement {

    pub fn new(die_elements: Vec<Die>, static_elements: Vec<i32>) -> DamageElement {
        DamageElement {
            die_elements: die_elements,
            static_elements: static_elements,
        }
    }

    fn parse_die_elements(notation: &str) -> Result<Vec<Die>, Box<dyn Error>> {
        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_die = Regex::new(r"\d+d\d+").unwrap();

        let capture: Vec<&str> = regex_die
            .find_iter(notation)
            .map(|m| m.as_str())
            .collect();

        let mut die_elements: Vec<Die> = Vec::new();

        for die_string in &capture {
            let tokens: Vec<&str> = die_string.split("d").collect();

            let n_die: i32 = match tokens[0].parse() {
                Ok(x) => x,
                _ => bail!(format!("Cannot parse die number '{}' to integer!", tokens[0])),
            };

            let s_die: i32 = match tokens[1].parse() {
                Ok(x) => x,
                _ => bail!(format!("Cannot parse die size '{}' to integer!", tokens[1])),
            };

            die_elements.push(
                Die::new(n_die, 1, s_die)
            );
        }

        Ok(die_elements)
    }

    fn parse_static_elements(notation: &str) -> Result<i32, Box<dyn Error>> {
        // The regex can only fail to compile on a system error, so safe to unwrap.
        let regex_statis = Regex::new(r"(\+\d+)|(-\d+)").unwrap();

        let capture: Vec<&str> = regex_statis
            .find_iter(notation)
            .map(|m| m.as_str())
            .collect();

        let mut running_total = 0;
        for value in &capture {
            let i = match value.parse() {
                Ok(x) => x,
                _ => 0 //bail!("asd")
            };
            running_total += i;
        }

        Ok(running_total)
    }

    pub fn from_notation_string(notation: &str) -> () { // Result<Vec<&str>, Box<dyn Error>>
        /* Uses regex to decompose the following elements from an input string:
                XdY
                +N
                -N
            Extracts as many of each as found, to allow for flexible inputs like a paladin
                using a 1d6 weapon with Improved Divine Strike (1d6 + 1d8 + mod).
        */

        let _die_elements = DamageElement::parse_die_elements(notation);
        //let regex_die = Regex::new(r"(?<die>\d+d\d+)(?<static>[-]?\d+)").unwrap();
        //let regex_die = Regex::new(r"(?<die>\d+d\d+)").unwrap();
        //let regex_static = Regex::new();

        //let Some(capture) = regex_die.captures(notation) else { None };

        /*
        let token_vector: Vec<&str> = notation.split(['d', '+']).collect();
        let (number_die, die_size, damage_modifier) = match token_vector.len() {
            3 => {
                let number_die = token_vector.get(0).unwrap();
                let die_size = token_vector.get(1).unwrap();
                let damage_modifier = token_vector.get(2).unwrap();
                (number_die, die_size, damage_modifier)
            },
            _ => simple_error::bail!(err_message),
        };

        let number_die: i32 = dpr_simulator::convert_value(number_die, &err_message)?;
        */

        //Ok(roll_elements)


    }

    /*
    pub fn roll_damage(&self, dice_roller: &mut ThreadRng) -> i32 {

        let mut dmg_roll: i32 = 0;
        for _ in 0..self.number_die {
            dmg_roll += dice_roller.gen_range(ROLL_MIN..self.max_roll);
        }

        dmg_roll + self.damage_modifier
    }

    pub fn roll_damage_crit(&self, dice_roller: &mut ThreadRng) -> i32 {

        let mut dmg_roll: i32 = 0;
        for _ in 0..self.number_die * 2 {
            dmg_roll += dice_roller.gen_range(ROLL_MIN..self.max_roll);
        }

        dmg_roll + self.damage_modifier
    }
    */

}

#[cfg(test)]
mod tests {
    use super::*;

/*
    fn _unpack_roll_vector(roll_capture: &Vec<i32>) -> (i32, i32) {

        let obs_min: i32 = *roll_capture.iter().min().unwrap();
        let obs_max: i32 = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }
*/

    #[test]
    fn test_parse_die_elements_single() {

        let obs_result = DamageElement::parse_die_elements("1d8+5");
        assert!(obs_result.is_ok());

        let exp_vector = vec![Die::new(1, 1, 8)];
        let obs_vector = obs_result.unwrap();
        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_die_elements_multiple() {

        let obs_result = DamageElement::parse_die_elements("1d6,1d8+5");
        assert!(obs_result.is_ok());

        let exp_vector = vec![Die::new(1, 1, 6), Die::new(1, 1, 8)];
        let obs_vector = obs_result.unwrap();
        assert_eq!(exp_vector, obs_vector);
    }

    #[test]
    fn test_parse_static_elements_single_pos() {

        let obs_result = DamageElement::parse_static_elements("1d8+5");
        assert!(obs_result.is_ok());
        assert_eq!(5, obs_result.unwrap());
    }

    #[test]
    fn test_parse_static_elements_single_neg() {

        let obs_result = DamageElement::parse_static_elements("1d8-5");
        assert!(obs_result.is_ok());
        assert_eq!(-5, obs_result.unwrap());
    }

    #[test]
    fn test_parse_static_elements_multiple() {

        let obs_result = DamageElement::parse_static_elements("1d8+5-3");
        assert!(obs_result.is_ok());
        assert_eq!(2, obs_result.unwrap());
    }  
/*
    #[test]
    fn test_from_notation_string_die() {
        // Test the extraction for die notation

        let obs_result = DamageElement::from_notation_string("1d8+5");
        println!("{:#?}", &obs_result);
        assert!(obs_result.is_ok());

        let capture = obs_result.unwrap();

        assert_eq!(&vec!["1d8"], capture["die"]);
    }
*/

    /*
    #[test]
    fn test_from_notation_string_positive_mod() {
        // Test the extraction for die notation

        let obs_result = DamageElement::from_notation_string("1d8+5");
        println!("{:#?}", &obs_result);
        assert!(obs_result.is_ok());

        assert_eq!(&vec!["+5"], &obs_result.unwrap());
    }

    #[test]
    fn test_from_notation_string_negative_mod() {
        // Test the extraction for die notation

        let obs_result = DamageElement::from_notation_string("1d8-5");
        println!("{:#?}", &obs_result);
        assert!(obs_result.is_ok());

        assert_eq!(&vec!["-5"], &obs_result.unwrap());
    }
    */

    /*
    #[test]
    fn test_create_empty() {

        let weapon = Weapon::create_empty();

        assert_eq!(weapon.number_die, 0);
        assert_eq!(weapon.max_roll, 1);
        assert_eq!(weapon.damage_modifier, 0);
    }

    #[test]
    fn test_from_notation_string_full() {

        // Test for the full '1d6+3' notation
        let exp_n = 1;
        let exp_max = 7;
        let exp_mod = 3;
        let input_string = format!("{}d{}+{}", exp_n, exp_max - 1, exp_mod);

        let w = Weapon::from_notation_string(&input_string).unwrap();
        assert_eq!(w.number_die, exp_n);
        assert_eq!(w.max_roll, exp_max);
        assert_eq!(w.damage_modifier, exp_mod);
    }

    #[test]
    fn test_from_notation_string_die() {

        // Test for the partial '1d6' notation
        let exp_n = 1;
        let exp_max = 7;
        let exp_mod = 0;
        let input_string = format!("{}d{}", exp_n, exp_max - 1);

        let w = Weapon::from_notation_string(&input_string).unwrap();
        assert_eq!(w.number_die, exp_n);
        assert_eq!(w.max_roll, exp_max);
        assert_eq!(w.damage_modifier, exp_mod);
    }

    #[test]
    fn test_from_notation_string_fail_1() {
        // First fail case - missing the 'd' from the expected notation.

        let result = Weapon::from_notation_string("16+3");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_notation_string_fail_2() {
        // Second fail case - includes a character which prevents i32 parsing

        let result = Weapon::from_notation_string("ad6+3");
        assert!(result.is_err());
    }

    #[test]
    fn test_roll_damage() {

        let damage_die: i32 = 8;
        let mut roller = rand::thread_rng();
        let w = Weapon::new(1, damage_die, 0);

        let mut roll_capture: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            roll_capture.push(w.roll_damage(&mut roller));
        }

        let (obs_min, obs_max) = unpack_roll_vector(&roll_capture);
        assert_eq!(obs_min, ROLL_MIN);
        assert_eq!(obs_max, damage_die);
    }

    #[test]
    fn test_roll_damage_crit() {

        let damage_die: i32 = 8;
        let mut roller = rand::thread_rng();
        let w = Weapon::new(1, damage_die, 0);

        let mut roll_capture: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            roll_capture.push(w.roll_damage_crit(&mut roller));
        }

        let (obs_min, obs_max) = unpack_roll_vector(&roll_capture);
        assert_eq!(obs_min, ROLL_MIN * 2);
        assert!(obs_max > damage_die);
    }
    */

}
