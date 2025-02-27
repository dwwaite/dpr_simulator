use crate::{HitResult, RollCollection};

#[derive(Debug, PartialEq)]
pub struct AttackProfile {
    pub target_ac: i32,
    hit_collection: Vec<RollCollection>,
    damage_collection: Vec<RollCollection>,
}

impl AttackProfile {
    pub fn new(
        target_ac: i32,
        hit_collection: Vec<RollCollection>,
        damage_collection: Vec<RollCollection>,
    ) -> AttackProfile {
        AttackProfile {
            target_ac,
            hit_collection,
            damage_collection,
        }
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
    /// // Create representation of a flat 1d20 roll to hit for a 1d8 weapon
    /// let hit_die = DiceBuilder::new().roll_max(20).build();
    /// let hit_collection = RollCollection::new(vec![hit_die], vec![]);
    ///
    /// let dmg_die = DiceBuilder::new().roll_max(8).build();
    /// let dmg_context = RollCollection::new(vec![dmg_die], vec![]);
    ///
    /// let attack_profile = AttackProfile::new(10, vec![hit_context], vec![dmg_context]);
    /// let (n_crits, n_hits, damage_dealt) = attack_profile.roll_turn(&mut roll_element);
    /// ```
    pub fn roll_turn(&mut self) -> (i32, i32, i32) {
        // Declare counters for the results - number of crits, number of hits, total damage
        let mut crit_counter = 0;
        let mut hit_counter = 0;
        let mut total_damage = 0;

        // For each hit/damage in the sequence, compute results
        for (hit_collection, dmg_collection) in self
            .hit_collection
            .iter_mut()
            .zip(self.damage_collection.iter_mut())
        {
            let hit_result = hit_collection.roll_against_armour_class(self.target_ac);
            total_damage += dmg_collection.roll_damage_result(&hit_result);

            match hit_result {
                HitResult::CriticalHit => {
                    crit_counter += 1;
                    hit_counter += 1;
                }
                HitResult::Hit => {
                    hit_counter += 1;
                }
                HitResult::Miss => (),
            }
        }
        (crit_counter, hit_counter, total_damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dice::DiceBuilder, static_modifier::StaticModifier, ModifierBehaviour, Ruleset};

    fn spawn_roll_collections(
        dice_pairs: Vec<(i32, i32)>,
        modifier_values: Vec<i32>,
        rule_mode: Ruleset,
    ) -> Vec<RollCollection> {
        let mut roll_collections: Vec<RollCollection> = Vec::new();
        for (dice_values, modifier_value) in dice_pairs.into_iter().zip(modifier_values.into_iter())
        {
            let (min_value, max_value) = dice_values;

            let die = DiceBuilder::new()
                .set_roll_min(min_value)
                .set_roll_max(max_value)
                .build();
            let modifier = StaticModifier::new(modifier_value, ModifierBehaviour::OnHit);

            roll_collections.push(RollCollection::new(vec![die], vec![modifier], rule_mode));
        }
        roll_collections
    }

    // region: roll_turn() single tests

    #[test]
    fn test_roll_turn_crit() {
        // Test the behaviour of the function when there is a single pair of dice in the attack profile which crit.
        let hit_collection = spawn_roll_collections(vec![(20, 20)], vec![0], Ruleset::DND5e);
        let damage_collection = spawn_roll_collections(vec![(1, 1)], vec![0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(1, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 1);
        assert_eq!(obs_hit, 1);
        assert_eq!(obs_dmg, 2);
    }

    #[test]
    fn test_roll_turn_hit() {
        // Test the behaviour of the function when there is a single pair of dice in the attack profile.
        let hit_collection = spawn_roll_collections(vec![(2, 5)], vec![0], Ruleset::DND5e);
        let damage_collection = spawn_roll_collections(vec![(1, 1)], vec![0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(1, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 0);
        assert_eq!(obs_hit, 1);
        assert_eq!(obs_dmg, 1);
    }

    #[test]
    fn test_roll_turn_miss() {
        // Test the behaviour of the function when there is a single pair of dice in the attack profile.
        let hit_collection = spawn_roll_collections(vec![(2, 5)], vec![0], Ruleset::DND5e);
        let damage_collection = spawn_roll_collections(vec![(1, 1)], vec![0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(10, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 0);
        assert_eq!(obs_hit, 0);
        assert_eq!(obs_dmg, 0);
    }

    // endregion:

    // region: roll_turn() multiple tests

    #[test]
    fn test_roll_turn_crit_multiple() {
        // Test the behaviour of the function when there are multiple pairs of dice which crit.
        let hit_collection =
            spawn_roll_collections(vec![(11, 12), (13, 14)], vec![0, 0], Ruleset::PF2e);
        let damage_collection =
            spawn_roll_collections(vec![(1, 1), (2, 2)], vec![0, 0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(1, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 2);
        assert_eq!(obs_hit, 2);
        assert_eq!(obs_dmg, 6);
    }

    #[test]
    fn test_roll_turn_hit_multiple() {
        // Test the behaviour of the function when there are multiple pairs of dice in the attack profile
        // and they all hit.
        let hit_collection =
            spawn_roll_collections(vec![(2, 5), (2, 6)], vec![0, 0], Ruleset::DND5e);
        let damage_collection =
            spawn_roll_collections(vec![(1, 1), (1, 1)], vec![0, 0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(1, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 0);
        assert_eq!(obs_hit, 2);
        assert_eq!(obs_dmg, 2);
    }

    #[test]
    fn test_roll_turn_miss_multiple() {
        // Test the behaviour of the function when there are multiple pairs of dice in the attack profile
        // and they all miss.
        let hit_collection =
            spawn_roll_collections(vec![(2, 5), (2, 6)], vec![0, 0], Ruleset::DND5e);
        let damage_collection =
            spawn_roll_collections(vec![(1, 1), (1, 1)], vec![0, 0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(10, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 0);
        assert_eq!(obs_hit, 0);
        assert_eq!(obs_dmg, 0);
    }

    // endregion:

    // region: roll_turn() mixed tests

    #[test]
    fn test_roll_turn_crit_mixed() {
        // Test the behaviour of the function when there are a mix of pairs of dice which crit, hit, and miss.
        let hit_collection = spawn_roll_collections(
            vec![(20, 20), (12, 14), (1, 2)],
            vec![0, 0, 0],
            Ruleset::DND5e,
        );
        let damage_collection =
            spawn_roll_collections(vec![(2, 2), (1, 1), (1, 1)], vec![0, 0, 0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(10, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 1);
        assert_eq!(obs_hit, 2);
        assert_eq!(obs_dmg, 5);
    }

    #[test]
    fn test_roll_turn_mixed() {
        // Test the behaviour of the function when there are multiple pairs of dice in the attack profile
        // and one hits, one misses
        let hit_collection =
            spawn_roll_collections(vec![(2, 5), (12, 14)], vec![0, 0], Ruleset::DND5e);
        let damage_collection =
            spawn_roll_collections(vec![(1, 1), (1, 1)], vec![0, 0], Ruleset::DND5e);

        let mut attack_profile = AttackProfile::new(10, hit_collection, damage_collection);
        let (obs_crit, obs_hit, obs_dmg) = attack_profile.roll_turn();

        assert_eq!(obs_crit, 0);
        assert_eq!(obs_hit, 1);
        assert_eq!(obs_dmg, 1);
    }

    // endregion:
}
