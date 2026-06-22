use crate::dice::Dice;
use crate::dice_collection::DiceCollection;
use crate::mutation_seed::MutationSeed;
use crate::static_modifier::StaticModifier;
use crate::{ApplyTrait, D20Value, RollBehaviour, RollKind, Ruleset};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct RollInstance {
    dice: Vec<DiceCollection>,
    modifiers: Vec<StaticModifier>,
}

impl RollInstance {
    pub fn roll_as_damage(&mut self, roll_kind: RollKind) -> i32 {
        let roll_total: i32 = self.dice.iter_mut().map(|x| x.roll(roll_kind)).sum();
        let mod_total: i32 = self.modifiers.iter().map(|x| x.roll(roll_kind)).sum();

        roll_total + mod_total
    }

    pub fn roll_as_hit(&mut self) -> (D20Value, i32) {
        let mut d20_value = D20Value::Normal;
        let mut roll_total = 0;

        for dc in self.dice.iter_mut() {
            // This is not an elegant implementation.
            let roll = dc.roll(RollKind::Normal);
            if (roll == 20) & (dc.n_die == 1) & (dc.dice.sides == 20) {
                d20_value = D20Value::Natural20;
            } else if (roll == 1) & (dc.n_die == 1) & (dc.dice.sides == 20) {
                d20_value = D20Value::Natural1;
            }
            roll_total += roll
        }

        let mod_total: i32 = self
            .modifiers
            .iter()
            .map(|x| x.roll(RollKind::Normal))
            .sum();

        (d20_value, roll_total + mod_total)
    }
}

#[derive(Debug, PartialEq)]
pub struct RollInstanceBuilder {
    dice: Vec<DiceCollection>,
    modifiers: Vec<StaticModifier>,
    rule_set: Ruleset,
}

impl RollInstanceBuilder {
    pub fn new(rule_set: Ruleset) -> RollInstanceBuilder {
        RollInstanceBuilder {
            dice: Vec::new(),
            modifiers: Vec::new(),
            rule_set: rule_set,
        }
    }

    fn parse_traits(notation: &str) -> Vec<(&str, i32)> {
        static RE_TRAITS: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\[(?P<text>[^\[\]0-9]*)(?P<suffix>\d+)?\]").unwrap());

        let mut result: Vec<(&str, i32)> = Vec::new();

        for cap in RE_TRAITS.captures_iter(notation) {
            let text = cap.name("text").map(|m| m.as_str()).unwrap();

            let suffix = cap
                .name("suffix")
                .and_then(|m| m.as_str().parse::<i32>().ok())
                .unwrap_or(0);

            result.push((text, suffix));
        }

        result
    }

    fn resolve_dice_behaviour(trait_options: &Vec<(&str, i32)>) -> RollBehaviour {
        for (trait_label, value) in trait_options {
            match trait_label.to_lowercase().as_str() {
                "kh" => {
                    return RollBehaviour::KeepHighFromX {
                        extra_rolls: *value,
                    }
                }
                "kl" => {
                    return RollBehaviour::KeepLowFromX {
                        extra_rolls: *value,
                    }
                }
                &_ => (), // No-op for unsupported traits
            }
        }

        RollBehaviour::Standard
    }

    /// Currently supported:
    /// 1. ApplyTrait::Deadly { extra_sides: i32 }
    /// 1. ApplyTrait::Fatal { upgraded_sides: i32 }
    /// 1. ApplyTrait::OnMissOnly
    /// 1. ApplyTrait::OnCriticalOnly { doubles_with_crit: bool }
    fn resolve_dice_trait(trait_options: &Vec<(&str, i32)>) -> ApplyTrait {
        for (trait_label, value) in trait_options {
            match trait_label.to_lowercase().as_str() {
                "deadly" => {
                    return ApplyTrait::Deadly {
                        extra_sides: *value,
                    }
                }
                "fatal" => {
                    return ApplyTrait::Fatal {
                        upgraded_sides: *value,
                    }
                }
                "oncrit" => {
                    return ApplyTrait::OnCriticalOnly {
                        doubles_with_crit: false,
                    }
                }
                "oncrit_doubles" => {
                    return ApplyTrait::OnCriticalOnly {
                        doubles_with_crit: true,
                    }
                }
                "onmiss" => return ApplyTrait::OnMissOnly,
                &_ => (), // No-op for unsupported traits
            }
        }

        ApplyTrait::Standard
    }

    fn resolve_modifier_trait(trait_options: &Vec<(&str, i32)>) -> ApplyTrait {
        for (trait_label, value) in trait_options {
            match trait_label.to_lowercase().as_str() {
                "oncrit" => {
                    return ApplyTrait::OnCriticalOnly {
                        doubles_with_crit: false,
                    }
                }
                "oncrit_doubles" => {
                    return ApplyTrait::OnCriticalOnly {
                        doubles_with_crit: true,
                    }
                }
                "onmiss" => return ApplyTrait::OnMissOnly,
                &_ => (), // No-op for unsupported traits
            }
        }

        ApplyTrait::Standard
    }

    /// Parse a dice notation string to extract the elements required for rolling.
    ///
    /// Extracts user information of dice to be represented in the roll and adds them
    /// to a borrowed vector of Dice. Accepts string in the form "NdS" where N is the
    /// number of dice to roll in the collection, and S is the size of the dice. The
    /// function then also searches for optional keywords contained in square brackets
    /// following an NdS pair.
    ///
    /// # Examples
    /// ```
    /// // ...
    /// ```
    fn parse_die_elements(&mut self, notation: &str, mut_seed: &mut MutationSeed) {
        // Use a lazy wrapper so that the expression is only compiled a single time.
        // Capture the NdS notation and also optional traits denoted by square brackets.
        // Traits are ignored in this capture, but required so that the die expression can
        // identify
        static RE_DICE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?P<count>\d+)d(?P<sides>\d+)(?P<tags>(?:\[[^\[\]]*\])*)").unwrap()
        });

        //if let Some(capture) = RE_DICE.captures(notation) {
        for capture in RE_DICE.captures_iter(notation) {
            // Directly unpack the die values from the capture
            let count: i32 = capture["count"].parse().unwrap();
            let sides: i32 = capture["sides"].parse().unwrap();

            // Process the tags through a more complex regex for trait values
            let tags = capture.name("tags").unwrap().as_str();
            let traits = RollInstanceBuilder::parse_traits(tags);

            let dice_mod = RollInstanceBuilder::resolve_dice_behaviour(&traits);
            let dc_mod = RollInstanceBuilder::resolve_dice_trait(&traits);

            let die = Dice::new(sides)
                .with_roll_seed(mut_seed)
                .with_roll_behaviour(dice_mod);

            self.dice
                .push(DiceCollection::new(count, die, dc_mod, self.rule_set));
        }
    }

    /// Extract tokens representing static damage modifiers from an input string.
    ///
    /// Extracts user information of static modifiers to be represented in the roll
    /// and adds them to a borrowed vector of StaticModifiers. Standard notation will
    /// be the +X modifier on an attack or damage roll, but negative values are also
    /// accepted (for example, MAP in Pathfinder). Also allows for multiple inputs in
    /// case of differing behaviours or just to simplify notation when multiple
    /// modifiers are being considered.
    ///
    /// # Examples
    /// ```
    /// // ...
    /// ```
    fn parse_static_elements(&mut self, notation: &str) {
        // Use a lazy wrapper so that the expression is only compiled a single time.
        static RE_MOD: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?P<modifier>[+-]\d+)(?P<tags>(?:\[[^\[\]]*\])*)").unwrap());

        for capture in RE_MOD.captures_iter(notation) {
            let mod_value: i32 = capture["modifier"].parse().unwrap();

            // Process the tags through a more complex regex for trait values
            let tags = capture.name("tags").unwrap().as_str();
            let traits = RollInstanceBuilder::parse_traits(tags);

            let mod_trait = RollInstanceBuilder::resolve_modifier_trait(&traits);

            self.modifiers
                .push(StaticModifier::new(mod_value, mod_trait, self.rule_set));
        }
    }

    /// Take a pair of input strings from the user and parse into the elements representing the roll
    ///
    /// Breaks and iterates over comma separation for dice elements, then identifiers and
    /// captures all static modifiers, identified through their +/- prefix.
    ///
    /// # Examples
    /// ```
    /// // ...
    /// ```
    pub fn parse_user_input(mut self, notation: &str, mut_seed: &mut MutationSeed) -> Self {
        self.parse_die_elements(notation, mut_seed);
        self.parse_static_elements(notation);

        self
    }

    pub fn build(self) -> RollInstance {
        RollInstance {
            dice: self.dice,
            modifiers: self.modifiers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roll_instance;

    // region: RollInstance::roll_as_damage tests

    #[test]
    fn test_roll_as_damage_case_normal_single() {
        let mut roll_instance = RollInstance {
            dice: vec![DiceCollection::default()],
            modifiers: vec![StaticModifier::default()],
        };

        // min(1d20) + 1 = 2
        let obs_result = roll_instance.roll_as_damage(RollKind::Normal);
        assert!(obs_result >= 2);
    }

    #[test]
    fn test_roll_as_damage_case_normal_multiple() {
        let mut roll_instance = RollInstance {
            dice: vec![
                DiceCollection::default(),
                DiceCollection::default(),
                DiceCollection::default(),
            ],
            modifiers: vec![StaticModifier::default(), StaticModifier::default()],
        };

        // min(1d20) + min(1d20) + min(1d20) + 1 + 1 = 5
        let obs_result = roll_instance.roll_as_damage(RollKind::Normal);
        assert!(obs_result >= 5);
    }

    #[test]
    fn test_roll_as_damage_case_critical_single() {
        let mut roll_instance = RollInstance {
            dice: vec![DiceCollection::default()],
            modifiers: vec![StaticModifier::default()],
        };

        // min(2d20) + 1 = 3 (D&D rules)
        let obs_result = roll_instance.roll_as_damage(RollKind::Critical);
        assert!(obs_result >= 3);
    }

    #[test]
    fn test_roll_as_damage_case_critical_multiple() {
        let mut roll_instance = RollInstance {
            dice: vec![
                DiceCollection::default(),
                DiceCollection::default(),
                DiceCollection::default(),
            ],
            modifiers: vec![StaticModifier::default(), StaticModifier::default()],
        };

        // min(2d20) + min(2d20) + min(2d20) + 1 + 1 = 5 (D&D rules)
        let obs_result = roll_instance.roll_as_damage(RollKind::Critical);
        assert!(obs_result >= 3);
    }

    #[test]
    fn test_roll_as_damage_case_miss() {
        let mut roll_instance = RollInstance {
            dice: vec![DiceCollection::default()],
            modifiers: vec![StaticModifier::default()],
        };

        let obs_result = roll_instance.roll_as_damage(RollKind::Miss);
        assert_eq!(0, obs_result);
    }

    // endregion:

    // region: RollInstance::roll_as_hit tests

    #[test]
    fn test_roll_as_hit_natural_20() {
        // For the current implementation of Dice/DiceCollection and MutationSeed, the first seed initialisation
        // value to yield a 20 of a 1d20 roll is 26.
        let mut mutation_seed = MutationSeed::new(Some(26));
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let (d20_value, obs_value) = roll_instance.roll_as_hit();

        assert_eq!(D20Value::Natural20, d20_value);
        assert_eq!(21, obs_value); // 1d20 + 1 at max
    }

    #[test]
    fn test_roll_as_hit_natural_1() {
        // For the current implementation of Dice/DiceCollection and MutationSeed, the first seed initialisation
        // value to yield a 20 of a 1d20 roll is 36.
        let mut mutation_seed = MutationSeed::new(Some(36));
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let (d20_value, obs_value) = roll_instance.roll_as_hit();

        assert_eq!(D20Value::Natural1, d20_value);
        assert_eq!(2, obs_value); // 1d20 + 1 at min
    }

    #[test]
    fn test_roll_as_hit_regular() {
        // For the current implementation of Dice/DiceCollection and MutationSeed, seeding at 3 rolls a value
        // of 14.
        let mut mutation_seed = MutationSeed::new(Some(3));
        let mut roll_instance = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d20+1", &mut mutation_seed)
            .build();

        let (d20_value, obs_value) = roll_instance.roll_as_hit();

        assert_eq!(D20Value::Normal, d20_value);
        assert_eq!(15, obs_value);
    }

    // endregion:

    // region: RollInstanceBuilder initialisation

    #[test]
    fn test_builder_init() {
        let exp_builder = RollInstanceBuilder {
            dice: Vec::new(),
            modifiers: Vec::new(),
            rule_set: Ruleset::DND5e,
        };

        let obs_builder = RollInstanceBuilder::new(Ruleset::DND5e);

        assert_eq!(exp_builder, obs_builder);
    }

    // endregion:

    // region: RollInstanceBuilder::parse_traits tests

    #[test]
    fn test_parse_traits_text_only() {
        let result = RollInstanceBuilder::parse_traits("drop[text]");
        assert_eq!(1, result.len());

        let obs_result = result.first();
        assert!(obs_result.is_some());

        let (obs_trait, obs_number) = obs_result.unwrap();
        assert_eq!(&"text", obs_trait);
        assert_eq!(0, *obs_number);
    }

    #[test]
    fn test_parse_traits_text_and_number() {
        let result = RollInstanceBuilder::parse_traits("drop[text12]");
        assert_eq!(1, result.len());

        let obs_result = result.first();
        assert!(obs_result.is_some());

        let (obs_trait, obs_number) = obs_result.unwrap();
        assert_eq!(&"text", obs_trait);
        assert_eq!(12, *obs_number);
    }

    #[test]
    fn test_parse_traits_no_value() {
        let obs_result = RollInstanceBuilder::parse_traits("drop");
        assert_eq!(0, obs_result.len());
    }

    #[test]
    fn test_parse_traits_multiple_entries() {
        let obs_result = RollInstanceBuilder::parse_traits("drop[text][more1][last4]");
        assert_eq!(3, obs_result.len());

        let exp_values = vec![("text", 0), ("more", 1), ("last", 4)];

        for (exp_value, obs_value) in exp_values.iter().zip(obs_result.iter()) {
            let (exp_text, exp_number) = exp_value;
            let (obs_text, obs_number) = obs_value;

            assert_eq!(exp_text, obs_text);
            assert_eq!(exp_number, obs_number);
        }
    }

    // endregion:

    // region: RollInstanceBuilder::resolve_dice_behaviour tests

    #[test]
    fn test_resolve_dice_behaviour_single_kh() {
        let input: Vec<(&str, i32)> = vec![("kh", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_behaviour(&input);

        assert_eq!(RollBehaviour::KeepHighFromX { extra_rolls: 2 }, obs_result);
    }

    #[test]
    fn test_resolve_dice_behaviour_single_lh() {
        let input: Vec<(&str, i32)> = vec![("kl", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_behaviour(&input);

        assert_eq!(RollBehaviour::KeepLowFromX { extra_rolls: 2 }, obs_result);
    }

    #[test]
    fn test_resolve_dice_behaviour_single_unsupported() {
        let input: Vec<(&str, i32)> = vec![("asd", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_behaviour(&input);

        assert_eq!(RollBehaviour::Standard, obs_result);
    }

    #[test]
    fn test_resolve_dice_behaviour_multiple_order() {
        let input: Vec<(&str, i32)> = vec![("kl", 2), ("kh", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_behaviour(&input);

        assert_eq!(RollBehaviour::KeepLowFromX { extra_rolls: 2 }, obs_result);
    }

    #[test]
    fn test_resolve_dice_behaviour_multiple_mixed() {
        let input: Vec<(&str, i32)> = vec![("asdasd", 2), ("kh", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_behaviour(&input);

        assert_eq!(RollBehaviour::KeepHighFromX { extra_rolls: 2 }, obs_result);
    }

    // endregion:

    // region: RollInstanceBuilder::resolve_dice_trait tests

    #[test]
    fn test_resolve_dice_trait_single_deadly() {
        let input: Vec<(&str, i32)> = vec![("deadly", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(ApplyTrait::Deadly { extra_sides: 2 }, obs_result);
    }

    #[test]
    fn test_resolve_dice_trait_single_fatal() {
        let input: Vec<(&str, i32)> = vec![("fatal", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(ApplyTrait::Fatal { upgraded_sides: 2 }, obs_result);
    }

    #[test]
    fn test_resolve_dice_trait_single_onmiss() {
        let input: Vec<(&str, i32)> = vec![("onmiss", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(ApplyTrait::OnMissOnly, obs_result);
    }

    #[test]
    fn test_resolve_dice_trait_single_oncrit_single() {
        let input: Vec<(&str, i32)> = vec![("oncrit", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_dice_trait_single_oncrit_singleoncrit_doubles() {
        let input: Vec<(&str, i32)> = vec![("oncrit_doubles", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_dice_trait_single_unsupported() {
        let input: Vec<(&str, i32)> = vec![("qweqwe", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(ApplyTrait::Standard, obs_result);
    }

    #[test]
    fn test_resolve_dice_trait_multiple_order() {
        let input: Vec<(&str, i32)> = vec![("oncrit_doubles", 2), ("onmiss", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_dice_trait_multiple_mixed() {
        let input: Vec<(&str, i32)> = vec![("asdasd", 2), ("onmiss", 2)];
        let obs_result = RollInstanceBuilder::resolve_dice_trait(&input);

        assert_eq!(ApplyTrait::OnMissOnly, obs_result);
    }

    // endregion:

    // region: RollInstanceBuilder::resolve_modifier_trait tests

    #[test]
    fn test_resolve_modifier_trait_single_onmiss() {
        let input: Vec<(&str, i32)> = vec![("onmiss", 0)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(ApplyTrait::OnMissOnly, obs_result);
    }

    #[test]
    fn test_resolve_modifier_trait_single_oncrit_single() {
        let input: Vec<(&str, i32)> = vec![("oncrit", 2)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_modifier_trait_single_oncrit_singleoncrit_doubles() {
        let input: Vec<(&str, i32)> = vec![("oncrit_doubles", 2)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_modifier_trait_single_unsupported() {
        let input: Vec<(&str, i32)> = vec![("fatal", 2)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(ApplyTrait::Standard, obs_result);
    }

    #[test]
    fn test_resolve_modifier_trait_multiple_order() {
        let input: Vec<(&str, i32)> = vec![("oncrit_doubles", 2), ("onmiss", 2)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            obs_result,
        );
    }

    #[test]
    fn test_resolve_modifier_trait_multiple_mixed() {
        let input: Vec<(&str, i32)> = vec![("kh", 2), ("onmiss", 2)];
        let obs_result = RollInstanceBuilder::resolve_modifier_trait(&input);

        assert_eq!(ApplyTrait::OnMissOnly, obs_result);
    }

    // endregion:

    // region: RollInstanceBuilder::parse_die_elements tests

    #[test]
    fn test_parse_die_elements_simple_die_dnd() {
        let dc = DiceCollection::new(1, Dice::new(4), ApplyTrait::Standard, Ruleset::DND5e);

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_simple_die_pf2e() {
        let dc = DiceCollection::new(1, Dice::new(4), ApplyTrait::Standard, Ruleset::PF2e);

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::PF2e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::PF2e);
        builder.parse_die_elements("1d4", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_set_seed() {
        let mut mut_seed = MutationSeed::new(Some(1));
        let exp_value = Dice::new(4).with_roll_seed(&mut mut_seed).roll(4);

        let mut mut_seed = MutationSeed::new(Some(4));
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4", &mut mut_seed);

        let obs_value = builder.build().roll_as_damage(RollKind::Normal);
        assert_eq!(exp_value, obs_value);
    }

    #[test]
    fn test_parse_die_elements_die_trait_deadly_single() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::Deadly { extra_sides: 6 },
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[deadly6]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_deadly_double() {
        let dc = DiceCollection::new(
            2,
            Dice::new(4),
            ApplyTrait::Deadly { extra_sides: 6 },
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("2d4[deadly6]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_fatal() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::Fatal { upgraded_sides: 8 },
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[fatal8]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_onmiss() {
        let dc = DiceCollection::new(1, Dice::new(4), ApplyTrait::OnMissOnly, Ruleset::DND5e);

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[onmiss]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_oncrit() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[oncrit]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_oncrit_doubles() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[oncrit_doubles]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_kh() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4).with_roll_behaviour(RollBehaviour::KeepHighFromX { extra_rolls: 2 }),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[kh2]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_kl() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4).with_roll_behaviour(RollBehaviour::KeepLowFromX { extra_rolls: 3 }),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[kl3]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_mixed_traits() {
        let dc = DiceCollection::new(1, Dice::new(4), ApplyTrait::OnMissOnly, Ruleset::DND5e);

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[onmiss][fatal6]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_mixed_roll_behaviours() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4).with_roll_behaviour(RollBehaviour::KeepLowFromX { extra_rolls: 2 }),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[kh6][kl2]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_die_trait_and_behaviour() {
        let dc = DiceCollection::new(
            1,
            Dice::new(4).with_roll_behaviour(RollBehaviour::KeepLowFromX { extra_rolls: 2 }),
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        );

        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(dc);

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[onmiss][kl2]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_die_elements_multiple_die() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.dice.push(DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        ));

        exp_builder.dice.push(DiceCollection::new(
            2,
            Dice::new(6).with_roll_behaviour(RollBehaviour::KeepLowFromX { extra_rolls: 2 }),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        ));

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_die_elements("1d4[onmiss],2d6[kl2]", &mut mut_seed);

        assert_eq!(exp_builder, builder);
    }

    // endregion:

    // region: RollInstanceBuilder::parse_static_elements tests

    #[test]
    fn test_parse_static_elements_simple_positive_dnd() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder
            .modifiers
            .push(StaticModifier::new(5, ApplyTrait::Standard, Ruleset::DND5e));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_simple_negative_dnd() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            -5,
            ApplyTrait::Standard,
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("-5");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_simple_positive_pf2e() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::PF2e);
        exp_builder
            .modifiers
            .push(StaticModifier::new(5, ApplyTrait::Standard, Ruleset::PF2e));

        let mut builder = RollInstanceBuilder::new(Ruleset::PF2e);
        builder.parse_static_elements("+5");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_simple_negative_pf2e() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::PF2e);
        exp_builder
            .modifiers
            .push(StaticModifier::new(-5, ApplyTrait::Standard, Ruleset::PF2e));

        let mut builder = RollInstanceBuilder::new(Ruleset::PF2e);
        builder.parse_static_elements("-5");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_trait_onmiss() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            5,
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5[onmiss]");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_trait_oncrit() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            5,
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5[oncrit]");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_trait_oncrit_doubles() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            5,
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5[oncrit_doubles]");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_mixed_traits() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            5,
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5[onmiss][oncrit_doubles]");

        assert_eq!(exp_builder, builder);
    }

    #[test]
    fn test_parse_static_elements_multiple_modifiers() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder.modifiers.push(StaticModifier::new(
            5,
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        ));
        exp_builder.modifiers.push(StaticModifier::new(
            -2,
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            Ruleset::DND5e,
        ));

        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        builder.parse_static_elements("+5[onmiss]-2[oncrit_doubles]");

        assert_eq!(exp_builder, builder);
    }

    // endregion:

    // region: RollInstanceBuilder::parse_user_input tests

    #[test]
    fn test_parse_user_input_simple() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder
            .modifiers
            .push(StaticModifier::new(5, ApplyTrait::Standard, Ruleset::DND5e));
        exp_builder.dice.push(DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        ));

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        let obs_builder = builder.parse_user_input("1d4+5", &mut mut_seed);

        assert_eq!(exp_builder, obs_builder);
    }

    #[test]
    fn test_parse_user_input_complex() {
        let mut exp_builder = RollInstanceBuilder::new(Ruleset::DND5e);
        exp_builder
            .modifiers
            .push(StaticModifier::new(5, ApplyTrait::Standard, Ruleset::DND5e));
        exp_builder.modifiers.push(StaticModifier::new(
            -2,
            ApplyTrait::OnMissOnly,
            Ruleset::DND5e,
        ));

        exp_builder.dice.push(DiceCollection::new(
            1,
            Dice::new(4),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        ));
        exp_builder.dice.push(DiceCollection::new(
            2,
            Dice::new(6),
            ApplyTrait::Deadly { extra_sides: 8 },
            Ruleset::DND5e,
        ));

        let mut mut_seed = MutationSeed::new(None);
        let mut builder = RollInstanceBuilder::new(Ruleset::DND5e);
        let obs_builder = builder.parse_user_input("1d4,2d6[deadly8]+5-2[onmiss]", &mut mut_seed);

        assert_eq!(exp_builder, obs_builder);
    }

    #[test]
    fn test_parse_user_input_seed_mutation() {
        // A test to confirm that the MutationSeed is being correctly called for the number
        // of die being made within the function.

        let mut mut_seed = MutationSeed::new(Some(1));
        let _ = RollInstanceBuilder::new(Ruleset::DND5e)
            .parse_user_input("1d4,2d6,3d8,4d7", &mut mut_seed);

        // Expected value of the mut seed is 5, then increments to 6 when called here
        assert_eq!(Some(6), mut_seed.next_seed());
    }

    // endregion:
}
