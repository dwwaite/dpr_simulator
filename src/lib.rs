use rand::Rng;
use rand::rngs::ThreadRng;
use std::error::Error;

#[macro_use]
extern crate simple_error;

const ROLL_MIN: i32 = 1;
const ROLL_MAX_D20: i32 = 21;

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
        Weapon::new(0, 0, 0)
    }

    fn convert_value(input_value: &str, err_message: &str) -> Result<i32, Box<dyn Error>> {

        let value: i32 = match input_value.parse() {
            Ok(x) => x,
            _ => bail!(err_message),
        };
        Ok(value)
    }

    pub fn from_notation_string(notation: &str) -> Result<Weapon, Box<dyn Error>> {
        /* Expected notation is in the standard '1d8+5' style. May replace this with a regex later.
        */

        let err_message = format!("Unable to parse input string '{}'!", notation);
        //let get_value = my_vector.get(1).unwrap();

        // Step 1 - extract the positions of interest
        let token_vector: Vec<&str> = notation.split(['d', '+']).collect();
        let (n_die, die_size, dmg_modifier) = match token_vector.len() {
            3 => {
                let n_die = token_vector.get(0).unwrap();
                let die_size = token_vector.get(1).unwrap();
                let dmg_modifier = token_vector.get(2).unwrap();
                (n_die, die_size, dmg_modifier)
            },
            _ => bail!(err_message),
        };

        // Step 2 - Attempt the type conversions to get to i32
        let n_die: i32 = Weapon::convert_value(n_die, &err_message)?;
        let die_size: i32 = Weapon::convert_value(die_size, &err_message)?;
        let dmg_modifier: i32 = Weapon::convert_value(dmg_modifier, &err_message)?;

        Ok(Weapon::new(n_die, die_size, dmg_modifier))
    }

    fn roll_damage(&self, dice_roller: &mut ThreadRng) -> i32 {

        let mut dmg_roll: i32 = 0;
        for _ in 0..self.number_die {
            dmg_roll += dice_roller.gen_range(ROLL_MIN..self.max_roll)
        }

        dmg_roll + self.damage_modifier
    }
}

#[derive(Debug)]
pub struct TurnSimulation {
    number_mh_attacks: i32,
    number_oh_attacks: i32,
    hit_modifier: i32,
    main_hand: Weapon,
    off_hand: Weapon,
    //modifier_options: Vec<bool>,
    dice_roller: ThreadRng,
}

impl TurnSimulation {

    pub fn new(number_mh_attacks: i32, number_oh_attacks: i32, hit_modifier: i32, main_hand: Weapon, off_hand: Weapon) -> TurnSimulation {
        TurnSimulation {
            number_mh_attacks: number_mh_attacks,
            number_oh_attacks: number_oh_attacks,
            hit_modifier: hit_modifier,
            main_hand: main_hand,
            off_hand: off_hand,
            //modifier_options: Vec::<bool>::new(),
            dice_roller: rand::thread_rng(),
        }
    }

    fn roll_attack(&mut self) -> i32 {
        self.dice_roller.gen_range(ROLL_MIN..ROLL_MAX_D20) + self.hit_modifier
    }

    pub fn roll_turn(&mut self, target_ac: i32) -> i32 {

        let mut dmg = 0;

        for _ in 0..self.number_mh_attacks {
            dmg += match self.roll_attack() >= target_ac {
                true => self.main_hand.roll_damage(&mut self.dice_roller),
                false => 0,
            };
        }

        for _ in 0..self.number_oh_attacks {
            dmg += match self.roll_attack() >= target_ac {
                true => self.off_hand.roll_damage(&mut self.dice_roller),
                false => 0,
            };
        }

        dmg
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

    // Weapon
    #[test]
    fn test_weapon_create_empty() {

        let weapon = Weapon::create_empty();

        assert_eq!(weapon.number_die, 0);
        assert_eq!(weapon.max_roll, 1);
        assert_eq!(weapon.damage_modifier, 0);
    }

    #[test]
    fn test_weapon_convert_value() {

        let input: &str = "3";
        let output = Weapon::convert_value(input, "something went wrong").unwrap();

        assert_eq!(output, 3);

    }

    #[test]
    fn test_weapon_convert_value_fail() {

        let input: &str = "a";
        let result = Weapon::convert_value(input, "something went wrong");
        assert!(result.is_err());

    }

    #[test]
    fn test_weapon_from_notation_string() {

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
    fn test_weapon_from_notation_string_fail_1() {
        // First fail case - missing the 'd' from the expected notation.

        let result = Weapon::from_notation_string("16+3");
        assert!(result.is_err());
    }

    #[test]
    fn test_weapon_from_notation_string_fail_2() {
        // Second fail case - missing the '+' from the expected notation.

        let result = Weapon::from_notation_string("1d63");
        assert!(result.is_err());
    }

    #[test]
    fn test_weapon_roll_damage() {

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

    // TurnSimulation
    #[test]
    fn test_turnsimulation_roll_attack() {

        let mut ts = TurnSimulation::new(
            0,
            0,
            0,
            Weapon::create_empty(),
            Weapon::create_empty()
        );

        let mut roll_capture: Vec<i32> = Vec::new();    
        for _ in 0..10000 {
            roll_capture.push(ts.roll_attack());
        }

        let (obs_min, obs_max) = unpack_roll_vector(&roll_capture);
        assert_eq!(obs_min, ROLL_MIN);
        assert_eq!(obs_max, ROLL_MAX_D20 - 1);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_mh() {

        let mut ts = TurnSimulation::new(
            1,
            0,
            1,
            Weapon::new(1, 6, 1),
            Weapon::create_empty()
        );
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_oh() {

        let mut ts = TurnSimulation::new(
            0,
            1,
            1,
            Weapon::create_empty(),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_both() {

        let mut ts = TurnSimulation::new(
            1,
            1,
            1,
            Weapon::new(1, 6, 1),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage >= 2);
    }

    #[test]
    fn test_turnsimulation_roll_turn_fail() {

        let mut ts = TurnSimulation::new(
            1,
            1,
            0,
            Weapon::new(1, 6, 1),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(21);
        assert_eq!(roll_damage, 0);
    }
}
