use rand::Rng;
use rand::rngs::ThreadRng;
use std::error::Error;

const ROLL_MIN: i32 = 1;

#[derive(Debug)]
pub struct Weapon {
    number_die: i32,
    max_roll: i32,
    damage_modifier: i32,
}

impl Weapon {

    pub fn new(number_die: i32, die_size: i32, damage_modifier: i32) -> Weapon {
        Weapon {
            number_die: number_die,
            max_roll: die_size + 1,
            damage_modifier: damage_modifier,
        }
    }

    pub fn create_empty() -> Weapon {
        let number_die = 0;
        let die_size = 0;
        let damage_modifier = 0;
        Weapon::new(number_die, die_size, damage_modifier)
    }

    pub fn from_notation_string(notation: &str) -> Result<Weapon, Box<dyn Error>> {
        // Expected notation is in the standard '1d8+5' style. May replace this with a regex later.

        let err_message = format!("Unable to parse input string '{}'!", notation);

        // Step 1 - extract the positions of interest
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

        // Step 2 - Attempt the type conversions to get to i32
        let number_die: i32 = dpr_simulator::convert_value(number_die, &err_message)?;
        let die_size: i32 = dpr_simulator::convert_value(die_size, &err_message)?;
        let damage_modifier: i32 = dpr_simulator::convert_value(damage_modifier, &err_message)?;

        Ok(Weapon::new(number_die, die_size, damage_modifier))
    }

    pub fn roll_damage(&self, dice_roller: &mut ThreadRng) -> i32 {

        let mut dmg_roll: i32 = 0;
        for _ in 0..self.number_die {
            dmg_roll += dice_roller.gen_range(ROLL_MIN..self.max_roll)
        }

        dmg_roll + self.damage_modifier
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
    fn test_create_empty() {

        let weapon = Weapon::create_empty();

        assert_eq!(weapon.number_die, 0);
        assert_eq!(weapon.max_roll, 1);
        assert_eq!(weapon.damage_modifier, 0);
    }

    #[test]
    fn test_from_notation_string() {

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
    fn test_from_notation_string_fail_1() {
        // First fail case - missing the 'd' from the expected notation.

        let result = Weapon::from_notation_string("16+3");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_notation_string_fail_2() {
        // Second fail case - missing the '+' from the expected notation.

        let result = Weapon::from_notation_string("1d63");
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
}