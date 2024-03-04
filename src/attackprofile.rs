use crate::{dicecontext::DiceContext, HitResult, Ruleset};
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq)]
pub struct AttackProfile {
    pub target_ac: i32,
    hit_dice: Vec<DiceContext>,
    damage_dice: Vec<DiceContext>,
    ruleset: Ruleset,
}

impl AttackProfile {
    pub fn new(
        target_ac: i32,
        hit_dice: Vec<DiceContext>,
        damage_dice: Vec<DiceContext>,
        ruleset: Ruleset,
    ) -> AttackProfile {
        AttackProfile {
            target_ac: target_ac,
            hit_dice: hit_dice,
            damage_dice: damage_dice,
            ruleset: ruleset,
        }
    }

    fn roll_5e_attack(
        target_ac: i32,
        hit_die: &DiceContext,
        roll_element: &mut ThreadRng,
    ) -> HitResult {
        // Determine whether or not the attack is a miss, hit, or critical hit.

        let d20_roll = hit_die.roll(roll_element);

        if d20_roll == 20 {
            return HitResult::CriticalHit;
        } else if d20_roll + hit_die.static_modifier >= target_ac {
            return HitResult::Hit;
        } else {
            return HitResult::Miss;
        }
    }

    fn roll_2e_attack(
        target_ac: i32,
        hit_die: &DiceContext,
        roll_element: &mut ThreadRng,
    ) -> HitResult {
        // Determine whether or not the attack is a miss, hit, or critical hit.

        let d20_roll = hit_die.roll(roll_element);
        let d20_total = d20_roll + hit_die.static_modifier;

        // Set a sucess counter, to allow for the nat1/nat20 adjustments.
        // Values are 0 = miss, 1 = hit, 2 = critical.
        let mut current_state: i32;
        if d20_total >= (target_ac + 10) {
            current_state = 2;
        } else if d20_total >= target_ac {
            current_state = 1;
        } else {
            current_state = 0;
        }

        // Apply final adjustments
        if d20_roll == 20 {
            current_state += 1;
        } else if d20_roll == 1 {
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

    fn determine_damage(
        damage_dice: &DiceContext,
        roll_element: &mut ThreadRng,
        state: &HitResult,
        ruleset: &Ruleset,
    ) -> i32 {
        // Calculate the damage to be added to the running total.

        match (state, ruleset) {
            (&HitResult::CriticalHit, &Ruleset::PF2e) => {
                damage_dice.roll_critical(roll_element) + 2 * damage_dice.static_modifier
            }
            (&HitResult::CriticalHit, &Ruleset::DND5e) => {
                damage_dice.roll_critical(roll_element) + damage_dice.static_modifier
            }
            (&HitResult::Hit, _) => damage_dice.roll(roll_element) + damage_dice.static_modifier,
            _ => 0,
        }
    }

    pub fn roll_turn(&self, roll_element: &mut ThreadRng) -> (i32, i32, i32) {
        // Iterate through each damage element, computing the total damage over the turn.

        // Declare counters for the results - number of crits, number of hits, total damage
        let mut crit_counter = 0;
        let mut hit_counter = 0;
        let mut total_damage = 0;

        // For each hit/damage in the sequence, compute results
        for (hit_dice, damage_dice) in self.hit_dice.iter().zip(self.damage_dice.iter()) {
            // Perform the attack roll
            let attack_result = match self.ruleset {
                Ruleset::DND5e => {
                    AttackProfile::roll_5e_attack(self.target_ac, &hit_dice, roll_element)
                }
                Ruleset::PF2e => {
                    AttackProfile::roll_2e_attack(self.target_ac, &hit_dice, roll_element)
                }
            };

            // Track the result then assign damage
            AttackProfile::track_hits(&attack_result, &mut crit_counter, &mut hit_counter);
            total_damage += AttackProfile::determine_damage(
                &damage_dice,
                roll_element,
                &attack_result,
                &self.ruleset,
            );
        }

        (crit_counter, hit_counter, total_damage)
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use crate::{dice::DiceCollection, Reroll};
    use super::*;

    //region Attack rolls

    #[test]
    fn test_roll_attack_dnd5e_miss() {
        // Test the result when the attack misses under D&D 5e mode.

        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_5e_attack(
            10,
            &DiceContext::parse_dice_string("1d2"),
            &mut roll_element,
        );

        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_dnd5e_hit() {
        // Test the result when the attack hits under D&D 5e mode.

        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_5e_attack(
            0,
            &DiceContext::parse_dice_string("1d2"),
            &mut roll_element,
        );

        assert_eq!(HitResult::Hit, obs_result);
    }

    #[test]
    fn test_roll_attack_dnd5e_critical() {
        // Test the result when the attack critically hits under D&D 5e mode.

        let mut roll_element = rand::thread_rng();

        let mut cheating_die = DiceCollection::new(1, 20, Reroll::Standard);
        cheating_die.increase_minimum(20);

        let obs_result = AttackProfile::roll_5e_attack(0, &DiceContext::new(vec![cheating_die], 0), &mut roll_element);
        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_miss_under() {
        // Test the result when the attack misses under Pathfinder 2e mode.

        let mut roll_element = rand::thread_rng();
        let obs_result = AttackProfile::roll_2e_attack(
            5,
            &DiceContext::parse_dice_string("1d2"),
            &mut roll_element,
        );
        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_miss_nat1() {
        // Test the result when the attack misses under Pathfinder 2e mode.

        let mut roll_element = rand::thread_rng();

        let obs_result = AttackProfile::roll_2e_attack(
            0,
            &DiceContext::new(vec![DiceCollection::new(1, 1, Reroll::Standard)], 1),
            &mut roll_element,
        );
        assert_eq!(HitResult::Miss, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_hit() {
        // Test the result when the attack hits under Pathfinder 2e mode.

        let mut roll_element = rand::thread_rng();

        let obs_result = AttackProfile::roll_2e_attack(
            0,
            &DiceContext::new(vec![DiceCollection::new(2, 2, Reroll::Standard)], 1),
            &mut roll_element,
        );

        assert_eq!(HitResult::Hit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_critical_over() {
        // Test the result when the attack critically hits under Pathfinder 2e mode
        //  due to the +10 rule

        let mut roll_element = rand::thread_rng();

        let obs_result = AttackProfile::roll_2e_attack(
            1,
            &DiceContext::new(vec![DiceCollection::new(2, 2, Reroll::Standard)], 10),
            &mut roll_element,
        );

        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    #[test]
    fn test_roll_attack_pf2e_critical_nat20() {
        // Test the result when the attack critically hits under Pathfinder 2e mode
        //  due to the +10 rule

        let mut roll_element = rand::thread_rng();

        let obs_result = AttackProfile::roll_2e_attack(
            19,
            &DiceContext::new(vec![DiceCollection::new(20, 20, Reroll::Standard)], 0),
            &mut roll_element,
        );

        assert_eq!(HitResult::CriticalHit, obs_result);
    }

    //endregion

    //region Crit and hit counter

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

    //endregion

    //region Damage calculator

    #[test]
    fn test_determine_damage_dnd5e_crit() {
        // Test how the damage results are computed using D&D 5e critical hit rules.

        let mut roll_element = rand::thread_rng();

        let obs_dmg = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::CriticalHit,
            &Ruleset::DND5e,
        );
        assert_eq!(obs_dmg, 3);
    }

    #[test]
    fn test_determine_damage_pf2e_crit() {
        // Test how the damage results are computed using Pathfinder 2e critical hit rules.

        let mut roll_element = rand::thread_rng();

        let obs_dmg = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::CriticalHit,
            &Ruleset::PF2e,
        );
        assert_eq!(obs_dmg, 4);
    }

    #[test]
    fn test_determine_damage_hit() {
        // Test how the damage results are computed using a regular hit for both rulsets.

        let mut roll_element = rand::thread_rng();

        let obs_dmg_dnd = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::Hit,
            &Ruleset::DND5e,
        );
        assert_eq!(obs_dmg_dnd, 2);

        let obs_dmg_pf = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::Hit,
            &Ruleset::PF2e,
        );
        assert_eq!(obs_dmg_pf, 2);
    }

    #[test]
    fn test_determine_damage_miss() {
        // Test how the damage results are computed on a miss for both rulsets.

        let mut roll_element = rand::thread_rng();

        let obs_dmg_dnd = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::Miss,
            &Ruleset::DND5e,
        );
        assert_eq!(obs_dmg_dnd, 0);

        let obs_dmg_pf = AttackProfile::determine_damage(
            &DiceContext::parse_dice_string("1d1+1"),
            &mut roll_element,
            &HitResult::Miss,
            &Ruleset::PF2e,
        );
        assert_eq!(obs_dmg_pf, 0);
    }

    //endregion

    #[test]
    fn test_roll_turn() {
        /* Test the behaviour of the function. Not an exhaustive test as all of the
         *  internal functions are tested above. It's hard to determine exact outputs,
         *  but with this set up the function is guaranteed to hit, so hit and damage
         *  counter will be >0.
         */

        let mut roll_element = rand::thread_rng();

        let ap = AttackProfile::new(
            1,
            vec![DiceContext::parse_dice_string("1d20")],
            vec![DiceContext::parse_dice_string("1d6")],
            Ruleset::DND5e,
        );

        let (_, obs_hit, obs_dmg) = ap.roll_turn(&mut roll_element);

        assert!(obs_hit > 0);
        assert!(obs_dmg > 0);
    }
}

//endregion
