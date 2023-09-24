use rand::Rng;
use rand::rngs::ThreadRng;

use crate::DamageElement;

const MIN_ROLL: i32 = 1;
const MAX_ROLL: i32 = 20;

#[derive(Debug)]
pub struct AttackProfile {
    pub target_ac: i32,
    number_mh_attacks: i32,
    number_oh_attacks: i32,
    hit_modifier: i32,
    main_hand: DamageElement,
    off_hand: DamageElement,
    dice_roller: ThreadRng,
}

impl PartialEq for AttackProfile {
    fn eq(&self, other: &AttackProfile) -> bool {

        let ac: bool = self.target_ac == other.target_ac;
        let mh: bool = self.number_mh_attacks == other.number_mh_attacks;
        let oh: bool = self.number_oh_attacks == other.number_oh_attacks;
        let hit: bool = self.hit_modifier == other.hit_modifier;
        let mh_d: bool = self.main_hand == other.main_hand;
        let oh_d: bool = self.off_hand == other.off_hand;

        ac & mh & oh & hit & mh_d & oh_d
    }
}

impl AttackProfile {

    pub fn new(target_ac: i32, number_mh_attacks: i32, number_oh_attacks: i32, hit_modifier: i32, main_hand: DamageElement, off_hand: DamageElement) -> AttackProfile {
        AttackProfile {
            target_ac: target_ac,
            number_mh_attacks: number_mh_attacks,
            number_oh_attacks: number_oh_attacks,
            hit_modifier: hit_modifier,
            main_hand: main_hand,
            off_hand: off_hand,
            dice_roller: rand::thread_rng(),
        }
    }

    fn roll_d20(&mut self) -> i32 {
        self.dice_roller.gen_range(0..MAX_ROLL) + MIN_ROLL
    }

    fn roll_attack(&mut self) -> (bool, bool) {
        let d20_roll = self.roll_d20();

        let is_crit = d20_roll == MAX_ROLL; 
        let is_hit = d20_roll + self.hit_modifier >= self.target_ac;

        // A critical is automatically a hit, so is for any reason a critical hit did not
        //  hit the target AC, still return true.
        (is_crit, is_crit | is_hit)
    }

    pub fn roll_turn(&mut self) -> (i32, i32) {

        let mut total_damage = 0;
        let mut crit_counter: i32 = 0;
    
        for _ in 0..self.number_mh_attacks {
            let (is_crit, is_hit) = self.roll_attack();

            if is_hit {
                total_damage += self.main_hand.roll_damage(&mut self.dice_roller, is_crit);
            }
            if is_crit {
                crit_counter += 1;
            }
        }

        for _ in 0..self.number_oh_attacks {
            let (is_crit, is_hit) = self.roll_attack();

            if is_hit {
                total_damage += self.off_hand.roll_damage(&mut self.dice_roller, is_crit);
            }
            if is_crit {
                crit_counter += 1;
            }
        }

        (crit_counter, total_damage)
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_roll_attack(input_profile: &mut AttackProfile, n_iterations: i32) -> (Vec<bool>, Vec<bool>) {
        // Run the AttackProfile.roll_attack() function a pre-determined number of times and return
        //  the results as Vec<bool> structs.

        let mut crit_capture: Vec<bool> = Vec::new();
        let mut hit_capture: Vec<bool> = Vec::new();

        for _ in 0..n_iterations {
            let (is_crit, is_hit) = input_profile.roll_attack();
            crit_capture.push(is_crit);
            hit_capture.push(is_hit);
        }

        (crit_capture, hit_capture)
    }

    fn capture_roll_turns(input_profile: &mut AttackProfile, n_iterations: i32) -> (Vec<i32>, Vec<i32>) {
        // Run the AttackProfile.roll_turn() function a pre-determined number of times and return
        //  the results as Vec<i32> structs.

        let mut crit_capture: Vec<i32> = Vec::new();
        let mut dmg_capture: Vec<i32> = Vec::new();

        for _ in 0..n_iterations {
            let (crit_count, dmg_total) = input_profile.roll_turn();
            crit_capture.push(crit_count);
            dmg_capture.push(dmg_total);
        }

        (crit_capture, dmg_capture)
    }

    fn sweep_vector_range(obs_values: &Vec<i32>, min_value: i32, max_value: i32) {
        // Check that expected values are found in the result

        for v in min_value..(max_value + 1) {
            assert!(obs_values.contains(&v));
        }
    }

    #[test]
    fn test_partialeq_true() {
        // Test the AttackProfile equality function where sides match.

        let left = AttackProfile::new(1, 1, 1, 1, DamageElement::create_empty(), DamageElement::create_empty());
        let right = AttackProfile::new(1, 1, 1, 1, DamageElement::create_empty(), DamageElement::create_empty());
    
        assert_eq!(left, right);
    }

    #[test]
    fn test_partialeq_false() {
        // Test the AttackProfile equality function where sides differ.

        let left = AttackProfile::new(1, 1, 1, 1, DamageElement::create_empty(), DamageElement::create_empty());
        let right = AttackProfile::new(2, 1, 1, 1, DamageElement::create_empty(), DamageElement::create_empty());
    
        assert_ne!(left, right);
    }

    #[test]
    fn test_roll_d20() {
        // Test the AttackProfile::roll_d20() function, ensuring that the roll bounds are correct.

        let mut ap = AttackProfile::new(
            0,
            0,
            0,
            0,
            DamageElement::create_empty(),
            DamageElement::create_empty()
        );

        let mut roll_capture: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            roll_capture.push(
                ap.roll_d20()
            );
        }

        assert_eq!(1, *roll_capture.iter().min().unwrap());
        assert_eq!(20, *roll_capture.iter().max().unwrap());
    }

    #[test]
    fn test_roll_attack() {
        // Test the AttackProfile::roll_attack() function, checking that crits and hits values
        //  are reported correctly in a case when the target AC is within rolling range.

        let mut ap = AttackProfile::new(
            10,
            0,
            0,
            0,
            DamageElement::create_empty(),
            DamageElement::create_empty()
        );

        let (crit_capture, hit_capture) = capture_roll_attack(&mut ap, 10_000);

        // Check the crit flag returns
        assert!(crit_capture.contains(&false));
        assert!(crit_capture.contains(&true));

        // Check the hit flag returns
        assert!(hit_capture.contains(&false));
        assert!(hit_capture.contains(&true));
    }

    #[test]
    fn test_roll_attack_fail() {
        // Test the AttackProfile::roll_attack() function, checking that crits and hits values
        //  are reported correctly in a case when the target AC is above the rolling range.

        let mut ap = AttackProfile::new(
            21,
            0,
            0,
            0,
            DamageElement::create_empty(),
            DamageElement::create_empty()
        );

        let (crit_capture, hit_capture) = capture_roll_attack(&mut ap, 10_000);

        // Check the crit flag returns
        assert!(crit_capture.contains(&false));
        assert!(crit_capture.contains(&true));

        // Check the hit flag returns - still expect successful hits due to critical chance
        assert!(hit_capture.contains(&false));
        assert!(hit_capture.contains(&true));
    }

    #[test]
    fn test_roll_turn_mainhand_single() {
        // Test the attack range over mainhand weapon, single attack.

        let mut ap = AttackProfile::new(
            0,
            1,
            0,
            0,
            DamageElement::from_notation_string("1d6+1"),
            DamageElement::create_empty()
        );

        let (crit_capture, dmg_capture) = capture_roll_turns(&mut ap, 10_000);

        // Check that expected values are found in the result
        //  0 or 1 crits per round
        //  1d6+1 = 2 -> 7 for non-crit damage
        sweep_vector_range(&crit_capture, 0, 1);
        sweep_vector_range(&dmg_capture, 2, 7);

    }

    #[test]
    fn test_roll_turn_mainhand_multiple() {
        // Test the attack range over mainhand weapon, multiple attacks.

        let mut ap = AttackProfile::new(
            0,
            2,
            0,
            0,
            DamageElement::from_notation_string("1d6+1"),
            DamageElement::create_empty()
        );

        let (crit_capture, dmg_capture) = capture_roll_turns(&mut ap, 10_000);

        // Check that expected values are found in the result
        //  0, 1, or 2 crits per round
        // 2 * 1d6+1 = 4 -> 14 for non-crit damage
        sweep_vector_range(&crit_capture, 0, 2);
        sweep_vector_range(&dmg_capture, 4, 14);
    }

    #[test]
    fn test_roll_turn_offhand_single() {
        // Test the attack range over offhand weapon, single attack.

        let mut ap = AttackProfile::new(
            0,
            0,
            1,
            0,
            DamageElement::create_empty(),
            DamageElement::from_notation_string("1d6+1")
        );

        let (crit_capture, dmg_capture) = capture_roll_turns(&mut ap, 10_000);

        // Check that expected values are found in the result
        //  0 or 1 crits per round
        //  1d6+1 = 2 -> 7 for non-crit damage
        sweep_vector_range(&crit_capture, 0, 1);
        sweep_vector_range(&dmg_capture, 2, 7);
    }

    // Roll for 2 + 1 (max of three attacks)
    #[test]
    fn test_roll_turn_all_hands() {
        // Test the attack range over 2x mainhand and 1x offhand weapon attacks.

        let mut ap = AttackProfile::new(
            0,
            2,
            1,
            0,
            DamageElement::from_notation_string("1d6+1"),
            DamageElement::from_notation_string("1d4")
        );

        let (crit_capture, dmg_capture) = capture_roll_turns(&mut ap, 100_000);

        // Check that expected values are found in the result
        //  0 to 3 crits per round
        //  1d6+1 + 1d4 = 5 -> 18 for non-crit damage
        sweep_vector_range(&crit_capture, 0, 3);
        sweep_vector_range(&dmg_capture, 5, 18);
    }

}

//endregion
