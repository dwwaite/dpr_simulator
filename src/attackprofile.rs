use crate::{damageelement::DamageElement, dice::Die, HitResult, Ruleset};
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq)]
pub struct AttackProfile {
    pub target_ac: i32,
    damage_elements: Vec<DamageElement>,
    ruleset: Ruleset,
}

impl AttackProfile {
    pub fn new(
        target_ac: i32,
        damage_elements: Vec<DamageElement>,
        ruleset: Ruleset,
    ) -> AttackProfile {
        AttackProfile {
            target_ac: target_ac,
            damage_elements: damage_elements,
            ruleset: ruleset,
        }
    }

    fn roll_5e_attack(
        d20: &Die,
        hit_modifier: i32,
        target_ac: i32,
        roll_element: &mut ThreadRng,
    ) -> HitResult {
        // Determine whether or not the attack is a miss, hit, or critical hit.

        let d20_roll = d20.roll(roll_element);

        // Evaluate the result
        if d20_roll == 20 {
            return HitResult::CriticalHit;
        } else if d20_roll + hit_modifier >= target_ac {
            return HitResult::Hit;
        } else {
            return HitResult::Miss;
        }
    }

    fn roll_2e_attack(
        d20: &Die,
        hit_modifier: i32,
        target_ac: i32,
        roll_element: &mut ThreadRng,
    ) -> HitResult {
        // Determine whether or not the attack is a miss, hit, or critical hit.

        let d20_roll = d20.roll(roll_element);
        let roll_result = d20_roll + hit_modifier;

        // Set a sucess counter, to allow for the nat1/nat20 adjustments.
        // Values are 0 = miss, 1 = hit, 2 = critical.
        let mut current_state: i32;
        if roll_result >= (target_ac + 10) {
            current_state = 2;
        } else if roll_result >= target_ac {
            current_state = 1;
        } else {
            current_state = 0;
        }

        // Apply final adjustments
        if d20_roll == 20 {
            current_state += 1;
        }
        if d20_roll == 1 {
            current_state -= 1;
        }

        // Assess and return
        match current_state {
            2 => HitResult::CriticalHit,
            1 => HitResult::Hit,
            _ => HitResult::Miss,
        }
    }

    fn track_hits(attack_result: &HitResult, crit_counter: &mut i32, hit_counter: &mut i32) {
        // Track any required changes in the attack roll.

        match attack_result {
            &HitResult::CriticalHit => {
                *crit_counter += 1;
                *hit_counter += 1;
            }
            &HitResult::Hit => *hit_counter += 1,
            _ => (),
        }
    }

    fn determine_damage(state: &HitResult, ruleset: &Ruleset, dmg_roll: i32, dmg_mod: i32) -> i32 {
        // Calculate the damage to be added to the running total.

        match (state, ruleset) {
            (&HitResult::CriticalHit, &Ruleset::PF2e) => 2 * (dmg_roll + dmg_mod),
            (&HitResult::CriticalHit, &Ruleset::DND53) => (2 * dmg_roll) + dmg_mod,
            (&HitResult::Hit, _) => dmg_roll + dmg_mod,
            _ => 0,
        }
    }

    pub fn roll_turn(&self, roll_element: &mut ThreadRng) -> (i32, i32, i32) {
        // Iterate through each damage element, computing the total damage over the turn.

        // Declare counters for the results - number of crits, number of hits, total damage
        let mut crit_counter = 0;
        let mut hit_counter = 0;
        let mut total_damage = 0;

        // Set the D20 for the turn
        let d20 = Die::new(1, 20);

        // For each hit/damage in the sequence, compute results
        for damage_element in &self.damage_elements {
            // Perform the roll
            let attack_result = match self.ruleset {
                Ruleset::DND53 => AttackProfile::roll_5e_attack(
                    &d20,
                    damage_element.to_hit,
                    self.target_ac,
                    roll_element,
                ),
                Ruleset::PF2e => AttackProfile::roll_2e_attack(
                    &d20,
                    damage_element.to_hit,
                    self.target_ac,
                    roll_element,
                ),
            };

            // Track the result and assign damage
            AttackProfile::track_hits(&attack_result, &mut crit_counter, &mut hit_counter);

            let (dmg_roll, dmg_mod) = damage_element.roll_damage(roll_element);
            total_damage +=
                AttackProfile::determine_damage(&attack_result, &self.ruleset, dmg_roll, dmg_mod);
        }

        (crit_counter, hit_counter, total_damage)
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    //region Attack rolls (D&D 5e)

    #[test]
    fn test_roll_attack_dnd5e_miss() {
        // Test the result when the attack misses under D&D 5e mode.

        let d20 = Die::new(1, 2);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_5e_attack(&d20, 0, 20, &mut roll_element);

        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_dnd5e_hit() {
        // Test the result when the attack hits under D&D 5e mode.

        let d20 = Die::new(1, 20);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_5e_attack(&d20, 1, 1, &mut roll_element);

        assert_eq!(HitResult::Hit, obs_result);
    }

    #[test]
    fn test_roll_attack_dnd5e_critical() {
        // Test the result when the attack critically hits under D&D 5e mode.

        let d20 = Die::new(20, 20);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_5e_attack(&d20, 1, 1, &mut roll_element);

        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    //endregion

    //region Attack rolls (Pathfinder 2e)

    #[test]
    fn test_roll_attack_pf2e_miss() {
        // Test the result when the attack misses under Pathfinder 2e mode.

        let d20 = Die::new(1, 2);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 0, 20, &mut roll_element);

        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_hit() {
        // Test the result when the attack hits under Pathfinder 2e mode.

        let d20 = Die::new(5, 6);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 0, 1, &mut roll_element);

        assert_eq!(HitResult::Hit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_critical_over() {
        // Test the result when the attack critically hits under Pathfinder 2e mode due to the +10 rule

        let d20 = Die::new(5, 6);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 10, 1, &mut roll_element);

        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_d20_increment_hit() {
        // Test the result when the attack state increments (miss -> hit) due to a nat20.

        let d20 = Die::new(20, 20);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 1, 25, &mut roll_element);

        assert_eq!(HitResult::Hit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_d20_increment_crit() {
        // Test the result when the attack state increments (miss -> hit) due to a nat20.

        let d20 = Die::new(20, 20);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 1, 21, &mut roll_element);

        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_d20_decrement_miss() {
        // Test the result when the attack state decreases (hit -> miss) due to a nat1.

        let d20 = Die::new(1, 1);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 10, 2, &mut roll_element);

        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_d20_decrement_hit() {
        // Test the result when the attack state decreases (crit -> hit) due to a nat1.

        let d20 = Die::new(1, 1);
        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(&d20, 15, 5, &mut roll_element);

        assert_eq!(HitResult::Hit, obs_result);
    }

    //endregion

    #[test]
    fn test_track_hits_crit() {
        // Test how the crit/hit counters increase on a critical hit result.

        let mut crit_count: i32 = 0;
        let mut hit_count: i32 = 0;

        AttackProfile::track_hits(&HitResult::CriticalHit, &mut crit_count, &mut hit_count);
        assert_eq!(crit_count, 1);
        assert_eq!(hit_count, 1);
    }

    #[test]
    fn test_track_hits_hit() {
        // Test how the crit/hit counters increase on a regular hit result.

        let mut crit_count: i32 = 0;
        let mut hit_count: i32 = 0;

        AttackProfile::track_hits(&HitResult::Hit, &mut crit_count, &mut hit_count);
        assert_eq!(crit_count, 0);
        assert_eq!(hit_count, 1);
    }

    #[test]
    fn test_track_hits_miss() {
        // Test how the crit/hit counters increase on a miss result.

        let mut crit_count: i32 = 0;
        let mut hit_count: i32 = 0;

        AttackProfile::track_hits(&HitResult::Miss, &mut crit_count, &mut hit_count);
        assert_eq!(crit_count, 0);
        assert_eq!(hit_count, 0);
    }

    #[test]
    fn test_determine_damage_dnd5e_crit() {
        // Test how the damage results are computed using D&D 5e critical hit rules.

        let obs_dmg =
            AttackProfile::determine_damage(&HitResult::CriticalHit, &Ruleset::DND53, 1, 1);
        assert_eq!(obs_dmg, 3);
    }

    #[test]
    fn test_determine_damage_pf2e_crit() {
        // Test how the damage results are computed using Pathfinder 2e critical hit rules.

        let obs_dmg =
            AttackProfile::determine_damage(&HitResult::CriticalHit, &Ruleset::PF2e, 1, 1);
        assert_eq!(obs_dmg, 4);
    }

    #[test]
    fn test_determine_damage_hit() {
        // Test how the damage results are computed using a regular hit for both rulsets.

        let obs_dmg_dnd = AttackProfile::determine_damage(&HitResult::Hit, &Ruleset::DND53, 1, 1);
        assert_eq!(obs_dmg_dnd, 2);

        let obs_dmg_pf = AttackProfile::determine_damage(&HitResult::Hit, &Ruleset::PF2e, 1, 1);
        assert_eq!(obs_dmg_pf, 2);
    }

    #[test]
    fn test_determine_damage_miss() {
        // Test how the damage results are computed on a miss for both rulsets.

        let obs_dmg_dnd = AttackProfile::determine_damage(&HitResult::Miss, &Ruleset::DND53, 1, 1);
        assert_eq!(obs_dmg_dnd, 0);

        let obs_dmg_pf = AttackProfile::determine_damage(&HitResult::Miss, &Ruleset::PF2e, 1, 1);
        assert_eq!(obs_dmg_pf, 0);
    }

    #[test]
    fn test_roll_turn() {
        // Test the behaviour of the function. Not an exhaustive test as all of the internal fucntions are tested above.

        // It's hard to determine exact outputs, but with this set up the function is guaranteed to hit, so hit
        // and damage counter will be >0.
        let de = DamageElement::new(10, vec![Die::new(1, 6)], 1);
        let ap = AttackProfile::new(10, vec![de], Ruleset::DND53);
        let mut roll_element = rand::thread_rng();

        let (_, obs_hit, obs_dmg) = ap.roll_turn(&mut roll_element);

        assert!(obs_hit > 0);
        assert!(obs_dmg > 0);
    }
}

//endregion
