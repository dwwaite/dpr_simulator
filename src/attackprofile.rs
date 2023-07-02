use rand::Rng;
use rand::rngs::ThreadRng;

use crate::Weapon;

const ROLL_MIN: i32 = 1;
const ROLL_MAX_D20: i32 = 21;

#[derive(Debug)]
pub struct AttackProfile {
    number_mh_attacks: i32,
    number_oh_attacks: i32,
    hit_modifier: i32,
    main_hand: Weapon,
    off_hand: Weapon,
    dice_roller: ThreadRng,
}

impl AttackProfile {

    pub fn new(number_mh_attacks: i32, number_oh_attacks: i32, hit_modifier: i32, main_hand: Weapon, off_hand: Weapon) -> AttackProfile {
        AttackProfile {
            number_mh_attacks: number_mh_attacks,
            number_oh_attacks: number_oh_attacks,
            hit_modifier: hit_modifier,
            main_hand: main_hand,
            off_hand: off_hand,
            dice_roller: rand::thread_rng(),
        }
    }

    fn roll_attack(&mut self) -> i32 {
        self.dice_roller.gen_range(ROLL_MIN..ROLL_MAX_D20) + self.hit_modifier
    }

    pub fn roll_turn(&mut self, target_ac: &i32) -> i32 {

        let mut dmg = 0;

        for _ in 0..self.number_mh_attacks {
            dmg += match self.roll_attack() >= *target_ac {
                true => self.main_hand.roll_damage(&mut self.dice_roller),
                false => 0,
            };
        }

        for _ in 0..self.number_oh_attacks {
            dmg += match self.roll_attack() >= *target_ac {
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

    #[test]
    fn test_roll_attack() {

        let mut ts = AttackProfile::new(
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
    fn test_roll_turn_succeed_mh() {

        let mut ts = AttackProfile::new(
            1,
            0,
            1,
            Weapon::new(1, 6, 1),
            Weapon::create_empty()
        );
        let roll_damage = ts.roll_turn(&0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_roll_turn_succeed_oh() {

        let mut ts = AttackProfile::new(
            0,
            1,
            1,
            Weapon::create_empty(),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(&0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_roll_turn_succeed_both() {

        let mut ts = AttackProfile::new(
            1,
            1,
            1,
            Weapon::new(1, 6, 1),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(&0);
        assert!(roll_damage >= 2);
    }

    #[test]
    fn test_roll_turn_fail() {

        let mut ts = AttackProfile::new(
            1,
            1,
            0,
            Weapon::new(1, 6, 1),
            Weapon::new(1, 6, 1)
        );
        let roll_damage = ts.roll_turn(&21);
        assert_eq!(roll_damage, 0);
    }
}
