use rand::Rng;
use rand::rngs::ThreadRng;

const ROLL_MIN: u8 = 1;
const ROLL_MAX_D20: u8 = 21;

#[derive(Debug)]
pub struct Weapon {
    number_die: u8,
    die_size: u8,
    max_roll: u8,
    damage_modifier: u8,
}

impl Weapon {

    pub fn new(number_die: u8, die_size: u8, damage_modifier: u8) -> Weapon {
        Weapon {
            number_die: number_die,
            die_size: die_size,
            max_roll: die_size + 1,
            damage_modifier: damage_modifier,
        }
    }

    fn roll_damage(&self, dice_roller: &mut ThreadRng) -> u8 {

        let mut dmg_roll = 0;
        for i in 0..self.number_die {
            dmg_roll += dice_roller.gen_range(ROLL_MIN..self.max_roll)
        }

        dmg_roll + self.damage_modifier
    }
}

#[derive(Debug)]
pub struct TurnSimulation {
    number_mh_attacks: u8,
    number_oh_attacks: u8,
    hit_modifier: u8,
    main_hand: Weapon,
    off_hand: Weapon,
    //modifier_options: Vec<bool>,
    dice_roller: ThreadRng,
}

impl TurnSimulation {

    pub fn new(number_mh_attacks: u8, number_oh_attacks: u8, hit_modifier: u8, main_hand: Weapon, off_hand: Weapon) -> TurnSimulation {
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

    fn roll_attack(&mut self) -> u8 {
        self.dice_roller.gen_range(ROLL_MIN..ROLL_MAX_D20) + self.hit_modifier
    }

    pub fn roll_turn(&mut self, target_ac: u8) -> u8 {

        let mut dmg = 0;

        for i in 0..self.number_mh_attacks {
            dmg += match self.roll_attack() >= target_ac {
                true => self.main_hand.roll_damage(&mut self.dice_roller),
                false => 0,
            };
        }

        for i in 0..self.number_oh_attacks {
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

    fn spawn_empty_weapon() -> Weapon {
        Weapon::new(0, 0, 0)
    }

    fn unpack_roll_vector(roll_capture: &Vec<u8>) -> (u8, u8) {

        let obs_min = *roll_capture.iter().min().unwrap();
        let obs_max = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }

    // Weapon
    #[test]
    fn test_weapon_roll_damage() {

        let damage_die = 8;
        let mut roller = rand::thread_rng();
        let w = Weapon::new(1, damage_die, 0);

        let mut roll_capture: Vec<u8> = Vec::new();
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

        let damage_die = 8;
        let mut ts = TurnSimulation::new(0, 0, 0, spawn_empty_weapon(), spawn_empty_weapon());

        let mut roll_capture: Vec<u8> = Vec::new();    
        for _ in 0..10000 {
            roll_capture.push(ts.roll_attack());
        }

        let (obs_min, obs_max) = unpack_roll_vector(&roll_capture);
        assert_eq!(obs_min, ROLL_MIN);
        assert_eq!(obs_max, ROLL_MAX_D20 - 1);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_mh() {

        let mut ts = TurnSimulation::new(1, 0, 1, Weapon::new(1, 6, 1), spawn_empty_weapon());
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_oh() {

        let mut ts = TurnSimulation::new(0, 1, 1, spawn_empty_weapon(), Weapon::new(1, 6, 1));
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage > 0);
    }

    #[test]
    fn test_turnsimulation_roll_turn_succeed_both() {

        let mut ts = TurnSimulation::new(1, 1, 1, Weapon::new(1, 6, 1), Weapon::new(1, 6, 1));
        let roll_damage = ts.roll_turn(0);
        assert!(roll_damage >= 2);
    }

    #[test]
    fn test_turnsimulation_roll_turn_fail() {

        let mut ts = TurnSimulation::new(1, 1, 0, Weapon::new(1, 6, 1), Weapon::new(1, 6, 1));
        let roll_damage = ts.roll_turn(21);
        assert_eq!(roll_damage, 0);
    }
}
