use crate::{D20Value, MutationSeed, RollInstance, RollInstanceBuilder, RollKind, Ruleset};

#[derive(Debug, PartialEq)]
pub struct AttackProfile {
    pub target_ac: i32,
    rule_set: Ruleset,
    hit_collection: Vec<RollInstance>,
    damage_collection: Vec<RollInstance>,
}

impl AttackProfile {
    pub fn new(target_ac: i32, rule_set: Ruleset) -> AttackProfile {
        AttackProfile {
            target_ac,
            rule_set,
            hit_collection: vec![],
            damage_collection: vec![],
        }
    }

    /// Assess a roll event against a target armour class under D&D 5e rules
    ///
    /// # Examples
    /// ```
    /// // Rolling 1d20+5
    /// // ...
    /// ```
    fn evaluate_hit_roll_dnd(roll_instance: &mut RollInstance, target_ac: i32) -> RollKind {
        let (d20_value, roll_value) = roll_instance.roll_as_hit();

        match d20_value {
            D20Value::Natural20 => RollKind::Critical,
            D20Value::Natural1 => RollKind::Miss,
            _ => {
                if roll_value >= target_ac {
                    RollKind::Normal
                } else {
                    RollKind::Miss
                }
            }
        }
    }

    /// Assess a roll event against a target armour class under Pathfinder 2e rules
    ///
    /// # Examples
    /// ```
    /// // Rolling 1d20+5
    /// // ...
    /// ```
    fn evaluate_hit_roll_pf2e(roll_instance: &mut RollInstance, target_ac: i32) -> RollKind {
        let (d20_value, roll_value) = roll_instance.roll_as_hit();

        // Set an adjustment according to whether or not a 1 or 20 was rolled.
        let adjustment = match d20_value {
            D20Value::Natural20 => 1,
            D20Value::Natural1 => -1,
            D20Value::Normal => 0,
        };

        // Evaluate the roll score and modify by the roll adjustment
        let roll_score = match roll_value - target_ac {
            diff if diff >= 10 => 2,
            diff if diff >= 0 => 1,
            _ => 0,
        } + adjustment;

        // Make the final evaluation and return
        match roll_score.clamp(0, 2) {
            2 => RollKind::Critical,
            1 => RollKind::Normal,
            _ => RollKind::Miss,
        }
    }

    pub fn add_attack(
        &mut self,
        hit_notation: &str,
        dmg_notation: &str,
        mut_seed: &mut MutationSeed,
    ) {
        let hit_profile = RollInstanceBuilder::new(self.rule_set)
            .parse_user_input(hit_notation, mut_seed)
            .build();

        let dmg_profile = RollInstanceBuilder::new(self.rule_set)
            .parse_user_input(dmg_notation, mut_seed)
            .build();

        self.hit_collection.push(hit_profile);
        self.damage_collection.push(dmg_profile);
    }

    /// Iterate through the hit/damage DiceContext pairs and return the damage dealt.
    ///
    /// Uses the internal AC value to test each hit against, then rolls damage according
    /// to the results. Stores a vector of hit and damage die representing multiple
    /// attacks per turn of combat. Records the number of critical/regular hits in the
    /// turn rolled for tallying purposes.
    ///
    /// # Examples
    /// ```
    /// // ...
    /// ```
    pub fn roll_turn(&mut self) -> (i32, i32, i32) {
        // Declare counters for the results - number of crits, number of hits, total damage
        let mut crit_counter = 0;
        let mut hit_counter = 0;
        let mut total_damage = 0;

        for (hit_instance, dmg_instance) in self
            .hit_collection
            .iter_mut()
            .zip(self.damage_collection.iter_mut())
        {
            let hit_result = match self.rule_set {
                Ruleset::DND5e => {
                    AttackProfile::evaluate_hit_roll_dnd(hit_instance, self.target_ac)
                }
                Ruleset::PF2e => {
                    AttackProfile::evaluate_hit_roll_pf2e(hit_instance, self.target_ac)
                }
            };

            match hit_result {
                RollKind::Critical => {
                    crit_counter += 1;
                    hit_counter += 1;
                }
                RollKind::Normal => hit_counter += 1,
                _ => (),
            };

            total_damage += dmg_instance.roll_as_damage(hit_result);
        }

        (crit_counter, hit_counter, total_damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_predictable_mut_seed(roll_target: D20Value) -> MutationSeed {
        // Create a MutationSeed set to return a predictable value when rolled on a 1d20 dice.
        // For a standard roll, the dice will roll 14 on first roll.
        match roll_target {
            D20Value::Natural20 => MutationSeed::new(Some(26)),
            D20Value::Natural1 => MutationSeed::new(Some(36)),
            _ => MutationSeed::new(Some(3)),
        }
    }

    // region: AttackProfile initialisation

    #[test]
    fn test_attackprofile_init() {
        let exp_profile = AttackProfile {
            target_ac: 10,
            rule_set: Ruleset::DND5e,
            hit_collection: vec![],
            damage_collection: vec![],
        };

        let obs_profile = AttackProfile::new(10, Ruleset::DND5e);

        assert_eq!(exp_profile, obs_profile);
    }

    // endregion:

    // region: AttackProfile::evaluate_hit_roll_dnd tests

    #[test]
    fn test_evaluate_hit_roll_dnd_critical() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural20);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let obs_roll = AttackProfile::evaluate_hit_roll_dnd(&mut roll_instance, 10);
        assert_eq!(obs_roll, RollKind::Critical);
    }

    #[test]
    fn test_evaluate_hit_roll_dnd_standard() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let obs_roll = AttackProfile::evaluate_hit_roll_dnd(&mut roll_instance, 10);
        assert_eq!(obs_roll, RollKind::Normal);
    }

    #[test]
    fn test_evaluate_hit_roll_dnd_miss_ac() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let obs_roll = AttackProfile::evaluate_hit_roll_dnd(&mut roll_instance, 16);
        assert_eq!(obs_roll, RollKind::Miss);
    }

    #[test]
    fn test_evaluate_hit_roll_dnd_miss_nat1() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural1);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let obs_roll = AttackProfile::evaluate_hit_roll_dnd(&mut roll_instance, 1);
        assert_eq!(obs_roll, RollKind::Miss);
    }

    // endregion:

    // region: AttackProfile::evaluate_hit_roll_pf2e tests

    #[test]
    fn test_evaluate_hit_roll_pf2e_critical() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 15 against 5, evaluate critical by overshoot
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 5);
        assert_eq!(obs_roll, RollKind::Critical);
    }

    #[test]
    fn test_evaluate_hit_roll_pf2e_critical_adjusted() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural20);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 21 against 20, evaluate critical by nat20
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 20);
        assert_eq!(obs_roll, RollKind::Critical);
    }

    #[test]
    fn test_evaluate_hit_roll_pf2e_standard() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 15 against 14, evaluate hit
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 14);
        assert_eq!(obs_roll, RollKind::Normal);
    }

    #[test]
    fn test_evaluate_hit_roll_pf2e_standard_adjusted() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural20);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 21 against 25, evaluate hit by nat20
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 25);
        assert_eq!(obs_roll, RollKind::Normal);
    }

    #[test]
    fn test_evaluate_hit_roll_pf2e_miss() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 15 against 16, evaluate miss
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 16);
        assert_eq!(obs_roll, RollKind::Miss);
    }

    #[test]
    fn test_evaluate_hit_roll_pf2e_miss_adjusted() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural1);
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::PF2e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        // Roll 2 against 2, evaluate miss by nat1
        let obs_roll = AttackProfile::evaluate_hit_roll_pf2e(&mut roll_instance, 2);
        assert_eq!(obs_roll, RollKind::Miss);
    }

    // endregion:

    // region: AttackProfile::add_attack tests

    #[test]
    fn test_add_attack_no_seed() {
        let hit_notation = "1d20+5";
        let dmg_notation = "1d8+3";

        let mut mut_seed = MutationSeed::new(None);

        let exp_hit = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(hit_notation, &mut mut_seed)
            .build();
        let exp_dmg = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(dmg_notation, &mut mut_seed)
            .build();

        let exp_profile = AttackProfile {
            target_ac: 10,
            rule_set: Ruleset::DND5e,
            hit_collection: vec![exp_hit],
            damage_collection: vec![exp_dmg],
        };

        let mut obs_profile = AttackProfile::new(10, Ruleset::DND5e);
        obs_profile.add_attack(hit_notation, dmg_notation, &mut mut_seed);

        assert_eq!(exp_profile, obs_profile);
    }

    #[test]
    fn test_add_attack_set_seed_short_profile() {
        let hit_notation = "1d20+5";
        let dmg_notation = "1d8+3";

        let mut mut_seed = MutationSeed::new(Some(10));

        let exp_roll_hit = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(hit_notation, &mut mut_seed)
            .build()
            .roll_as_damage(RollKind::Normal);
        let exp_roll_damage = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(dmg_notation, &mut mut_seed)
            .build()
            .roll_as_damage(RollKind::Normal);

        let mut mut_seed = MutationSeed::new(Some(10));
        let mut profile = AttackProfile::new(10, Ruleset::DND5e);
        profile.add_attack(hit_notation, dmg_notation, &mut mut_seed);

        let obs_roll_hit = profile
            .hit_collection
            .first_mut()
            .unwrap()
            .roll_as_damage(RollKind::Normal);
        let obs_roll_damage = profile
            .damage_collection
            .first_mut()
            .unwrap()
            .roll_as_damage(RollKind::Normal);

        assert_eq!(exp_roll_hit, obs_roll_hit);
        assert_eq!(exp_roll_damage, obs_roll_damage);
    }

    #[test]
    fn test_add_attack_set_seed_long_profile() {
        let hit_notation = "1d20,1d30+5";
        let dmg_notation = "1d8,1d4+3";

        let mut mut_seed = MutationSeed::new(Some(10));

        let exp_roll_hit = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(hit_notation, &mut mut_seed)
            .build()
            .roll_as_damage(RollKind::Normal);
        let exp_roll_damage = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input(dmg_notation, &mut mut_seed)
            .build()
            .roll_as_damage(RollKind::Normal);

        let mut mut_seed = MutationSeed::new(Some(10));
        let mut profile = AttackProfile::new(10, Ruleset::DND5e);
        profile.add_attack(hit_notation, dmg_notation, &mut mut_seed);

        let obs_roll_hit = profile
            .hit_collection
            .first_mut()
            .unwrap()
            .roll_as_damage(RollKind::Normal);
        let obs_roll_damage = profile
            .damage_collection
            .first_mut()
            .unwrap()
            .roll_as_damage(RollKind::Normal);

        assert_eq!(exp_roll_hit, obs_roll_hit);
        assert_eq!(exp_roll_damage, obs_roll_damage);
    }

    #[test]
    fn test_add_attack_multiple_calls() {
        let mut mut_seed = MutationSeed::new(None);
        let mut obs_profile = AttackProfile::new(10, Ruleset::DND5e);

        obs_profile.add_attack("", "", &mut mut_seed);
        obs_profile.add_attack("", "", &mut mut_seed);

        assert_eq!(2, obs_profile.hit_collection.len());
        assert_eq!(2, obs_profile.damage_collection.len());
    }

    // endregion:

    // region: AttackProfile::roll_turn tests

    #[test]
    fn test_roll_turn_crit_counter_single() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural20);
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mutation_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(1, obs_crit);
        assert_eq!(1, obs_hit);
        assert_eq!(3, obs_dmg);
    }

    #[test]
    fn test_roll_turn_normal_counter_single() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mutation_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(0, obs_crit);
        assert_eq!(1, obs_hit);
        assert_eq!(2, obs_dmg);
    }

    #[test]
    fn test_roll_turn_miss_counter_single() {
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Natural1);
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mutation_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(0, obs_crit);
        assert_eq!(0, obs_hit);
        assert_eq!(0, obs_dmg);
    }

    #[test]
    fn test_roll_turn_multiple_crit() {
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural20);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural20);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural20);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(3, obs_crit);
        assert_eq!(3, obs_hit);
        assert_eq!(9, obs_dmg);
    }

    #[test]
    fn test_roll_turn_multiple_hit() {
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Normal);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Normal);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Normal);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(0, obs_crit);
        assert_eq!(3, obs_hit);
        assert_eq!(6, obs_dmg);
    }

    #[test]
    fn test_roll_turn_multiple_miss() {
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural1);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural1);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let mut mut_seed = create_predictable_mut_seed(D20Value::Natural1);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mut_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(0, obs_crit);
        assert_eq!(0, obs_hit);
        assert_eq!(0, obs_dmg);
    }

    #[test]
    fn test_roll_turn_multiple_mixed() {
        let mut attack_profile = AttackProfile::new(15, Ruleset::DND5e);

        let mut crit_seed = create_predictable_mut_seed(D20Value::Natural20);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut crit_seed);

        let mut std_seed = create_predictable_mut_seed(D20Value::Normal);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut std_seed);

        let mut miss_seed = create_predictable_mut_seed(D20Value::Natural1);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut miss_seed);

        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(1, obs_crit);
        assert_eq!(2, obs_hit);
        assert_eq!(5, obs_dmg);
    }

    #[test]
    fn test_roll_turn_pf2e_route() {
        // Non-exhaustive test, just to confirm that setting the AttackProfile rule routes correctly.
        let mut mutation_seed = create_predictable_mut_seed(D20Value::Normal);
        let mut attack_profile = AttackProfile::new(5, Ruleset::PF2e);
        attack_profile.add_attack("1d20+1", "1d1+1", &mut mutation_seed);

        // Result should critical by rolling 15 against AC5, then roll a Pathfinder critical for damage.
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();
        assert_eq!(1, obs_crit);
        assert_eq!(1, obs_hit);
        assert_eq!(4, obs_dmg);
    }

    // endregion:
}
