use rand::Rng;
use rand::rngs::ThreadRng;

const ROLL_MIN: i32 = 1;
const ROLL_MAX: i32 = 21;

#[derive(Debug)]
pub struct TurnSimulation {
    number_attacks: i32,
    weapon_die: i32,
    hit_modifier: i32,
    damage_modifier: i32,
    damage_roll_max: i32,
    modifier_options: Vec<bool>,
    dice_roller: ThreadRng,
}

impl TurnSimulation {

    pub fn new(number_attacks: i32, weapon_die: i32, hit_modifier: i32, damage_modifier: i32) -> TurnSimulation {
        TurnSimulation {
            number_attacks: number_attacks,
            weapon_die: weapon_die,
            damage_roll_max: weapon_die + 1,
            hit_modifier: hit_modifier,
            damage_modifier: damage_modifier,
            modifier_options: Vec::<bool>::new(),
            dice_roller: rand::thread_rng(),
        }
    }

    fn roll_attack(&mut self) -> i32 {
        self.dice_roller.gen_range(ROLL_MIN..ROLL_MAX) + self.hit_modifier
    }

    fn roll_damage(&mut self) -> i32 {
        self.dice_roller.gen_range(ROLL_MIN..self.weapon_die + 1) + self.damage_modifier
    }

    pub fn roll(&mut self, target_ac: i32) -> i32 {

        match self.roll_attack() >= target_ac {
            true => self.roll_damage(),
            false => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_attack() {

        let mut ts = TurnSimulation::new(0, 0, 0, 0);
        let mut roll_capture: Vec<i32> = Vec::new();
    
        for _ in 0..10000 {
            roll_capture.push(
                ts.roll_attack()
            );
        }
    
        assert_eq!(
            *roll_capture.iter().min().unwrap(),
            ROLL_MIN
        );
        assert_eq!(
            *roll_capture.iter().max().unwrap(),
            ROLL_MAX - 1
        );
    }

    #[test]
    fn test_roll_damage() {

        let damage_die = 8;

        let mut ts = TurnSimulation::new(0, damage_die, 0, 0);
        let mut roll_capture: Vec<i32> = Vec::new();

        for _ in 0..10000 {
            roll_capture.push(
                ts.roll_damage()
            );
        }
    
        assert_eq!(
            *roll_capture.iter().min().unwrap(),
            ROLL_MIN
        );
        assert_eq!(
            *roll_capture.iter().max().unwrap(),
            damage_die
        );
    }
}
