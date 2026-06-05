use std::cmp::Ordering;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::dice::Dice;
use crate::Ruleset;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RollKind {
    Normal,
    Critical,
    Miss,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApplyTrait {
    Standard,
    Deadly { extra_sides: i32, extra_dice: i32 },
    Fatal { upgraded_sides: i32 },
    OnMissOnly,
    OnCriticalOnly { doubles_with_crit: bool },
}

/// A representation of a collection of dice subject to roll conditions
#[derive(Debug)]
pub struct DiceCollection {
    n_die: i32,
    dice: Dice,
    roll_trait: ApplyTrait,
    rule_mode: Ruleset,
}

// Implement PartialEq to invoke the Dice PartialEq.
impl PartialEq for DiceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.n_die == other.n_die
            //&& self.dice == other.dice
            && self.roll_trait == other.roll_trait
            && self.rule_mode == other.rule_mode
    }
}

impl DiceCollection {
    /// Creates a new DiceCollection representation of a roll event.
    ///
    /// This collection contains an arbitrary number of dice with a modifier trait
    /// and the ruleset under which it rolls. This accounts for complex Pathfinder
    /// weapon properties like Fatal and Deadly, or for effects where a dice may
    /// get rolled only on a miss.
    /// 
    /// This struct is flexible enough to account for both weapon damage dice, and
    /// also standard 1d20+MOD hit conditions.
    ///
    /// # Examples
    /// ```
    /// let my_collection = DiceCollection::new(
    ///     2,
    ///     Dice::new(4, None),
    ///     ApplyTrait::Standard,
    ///     RuleSet::DND5e,
    /// );
    /// ```
    pub fn new(
        n_die: i32,
        dice: Dice,
        roll_trait: ApplyTrait,
        rule_set: Ruleset,
    ) -> DiceCollection {
        DiceCollection {
            n_die: n_die,
            dice: dice,
            roll_trait: roll_trait,
            rule_mode: rule_set,
        }
    }

    /// A helper function to smooth over rolling branches when rolling criticals
    fn roll_sum(&mut self, n_dice: i32, n_sides: i32) -> i32 {
        (0..n_dice)
            .map(|_| self.dice.roll(n_sides))
            .sum()
    }

    fn roll_standard(&mut self) -> i32 {
        // Catch cases where a regular would be ignored, otherwise return a standard roll
        match self.roll_trait {
            ApplyTrait::OnCriticalOnly { doubles_with_crit } => 0,
            ApplyTrait::OnMissOnly => 0,
            _ => self.dice.roll(self.dice.sides),
        }
    }

    fn roll_miss(&mut self) -> i32 {
        // Catch case where roll occurs on miss, otherwise return 0
        match self.roll_trait {
            ApplyTrait::OnMissOnly => self.dice.roll(self.dice.sides),
            _ => 0,
        }
    }

    fn roll_critical(&mut self) -> i32 {
        match self.roll_trait {
            // Universal traits across both rule sets
            ApplyTrait::OnCriticalOnly { doubles_with_crit } => {
                if(doubles_with_crit) {
                    self.roll_sum(self.n_die * 2, self.dice.sides)
                } else {
                    self.roll_sum(self.n_die, self.dice.sides)
                }
            },
            ApplyTrait::OnMissOnly => 0,
            // Pathfinder critical traits
            ApplyTrait::Deadly { extra_sides, extra_dice } => {
                //https://2e.aonprd.com/Traits.aspx?ID=570
                self.roll_sum(self.n_die * 2, self.dice.sides) + 
                    self.roll_sum(extra_dice, extra_sides)
            },
            ApplyTrait::Fatal { upgraded_sides } => {
                //https://2e.aonprd.com/Traits.aspx?ID=597
                self.roll_sum((self.n_die * 2) + 1, upgraded_sides)
            },
            // Default case
            _ => self.roll_sum(self.n_die * 2, self.dice.sides)
        }
    }

    pub fn roll(&mut self, kind: RollKind) -> i32 {

        match kind {
            RollKind::Normal => self.roll_standard(),
            RollKind::Critical => self.roll_critical(),
            RollKind::Miss => self.roll_miss(),
        }
    }

    /*
    /// Assess a roll event against a target armour class under D&D 5e rules
    ///
    /// # Examples
    /// ```
    /// // Rolling 1d20+5
    /// let mut roll_collection = RollCollection::new(
    ///     vec![DiceBuilder::new().set_roll_max(20).build()],
    ///     vec![StaticModifier::new(5, EvalBehaviour::OnHit)],
    ///     Ruleset::DND5e
    /// );
    /// let target_ac = 15;
    /// let result = roll_collection.eval_ac_roll_dnd(target_ac);
    /// ```
    fn eval_ac_roll_dnd(&mut self, target_ac: i32) -> HitResult {
        let mut roll_total: i32 = self.modifiers.iter().map(|x| x.evaluate_result(None)).sum();

        // Roll the dice and record if there is a modifier
        for die in &mut self.dice {
            let result = die.evaluate_result(None);

            if (result, die.max) == (20, 20) {
                return HitResult::CriticalHit;
            }

            roll_total += result;
        }

        if roll_total >= target_ac {
            HitResult::Hit
        } else {
            HitResult::Miss
        }
    }

    /// Assess a roll event against a target armour class under Pathfinder 2e rules
    ///
    /// # Examples
    /// ```
    /// // Rolling 1d20+5
    /// let mut roll_collection = RollCollection::new(
    ///     vec![DiceBuilder::new().set_roll_max(20).build()],
    ///     vec![StaticModifier::new(5, EvalBehaviour::OnHit)],
    ///     Ruleset::PF2e
    /// );
    /// let target_ac = 15;
    /// let result = roll_collection.eval_ac_roll_pathfinder(target_ac);
    /// ```
    fn eval_ac_roll_pathfinder(&mut self, target_ac: i32) -> HitResult {
        // Using a numeric value to represent the success state of the roll, so that it can be increased or decreased
        // in light of nat20 or nat1 rolls.

        let mut roll_total: i32 = self.modifiers.iter().map(|x| x.evaluate_result(None)).sum();
        let mut success_modifier = 0;
        let success_state: i32;

        // Roll the dice and record if there is a modifier
        for die in &mut self.dice {
            let result = die.evaluate_result(None);

            match (result, die.max) {
                (20, 20) => success_modifier += 1,
                (1, 20) => success_modifier -= 1,
                (_, _) => (),
            }

            roll_total += result;
        }

        // Evaluate the flat roll
        let roll_difference: i32 = roll_total - target_ac;

        if roll_difference >= 10 {
            success_state = 2;
        } else if roll_difference >= 0 {
            success_state = 1;
        } else {
            success_state = 0;
        }

        // Evaluate and return
        match (success_modifier + success_state).cmp(&1) {
            Ordering::Greater => HitResult::CriticalHit,
            Ordering::Equal => HitResult::Hit,
            _ => HitResult::Miss,
        }
    }
    */

    /*
    /// Parse a dice notation string to extract the elements required for rolling.
    ///
    /// Extracts user information of dice to be represented in the roll and adds them
    /// to a borrowed vector of Dice. Accepts string in the form "NdS" where N is the
    /// number of dice to roll in the collection, and S is the size of the dice. There
    /// are also two optional modifiers accepted, modulating one or both of the following:
    ///
    /// 1. Adding reroll mechanics - Advantage, Disadvantage, or 'double advantage',
    ///    which is effectively the Elven Accuracy mechanic from D&D 5E.
    /// 2. Adding Fatal rolling mechanics, from Pathfinder.
    ///
    /// # Examples
    /// ```
    /// // Regular roll for 2d6
    /// let mut dice_collection: Vec<Dice> = Vec::new();
    /// parse_die_element(&mut dice_collection, "2d6");
    ///
    /// // A d20 attack roll with advantage or disadvantage
    /// parse_die_element(&mut dice_collection, "1d20A");
    /// parse_die_element(&mut dice_collection, "1d20D");
    ///
    /// // Rolling with Elven Advantage mechanics
    /// parse_die_element(&mut dice_collection, "1d20AA");
    ///
    /// // Rolling a Pathfinder Dueling Pistol, standard or with Advantage
    /// parse_die_element(&mut dice_collection, "1d6~10");
    /// parse_die_element(&mut dice_collection, "1d6A~10");
    /// ```
    fn parse_die_elements(dice_vector: &mut Vec<Dice>, notation: &str) {
        // Use a lazy wrapper so that the expression is only compiled a single time.
        static RE_DICE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?P<n_dice>\d+)d(?P<die_size>\d+)(?P<behaviour>AA|A|D)?(?:~(?P<fatal>\d+))?",
            )
            .unwrap()
        });

        if let Some(capture) = RE_DICE.captures(notation) {
            // X in XdY~Z
            let n_dice: i32 = capture
                .name("n_dice")
                .and_then(|m| m.as_str().parse::<i32>().ok())
                .unwrap();

            // Y in XdY~Z
            let die_size: i32 = capture
                .name("die_size")
                .and_then(|m| m.as_str().parse::<i32>().ok())
                .unwrap();

            // Optional behaviour modifier in XdYmod~Z
            let adv_mod = capture.name("behaviour").map_or("", |m| m.as_str());
            let fatal_mod = capture
                .name("fatal")
                .and_then(|m| m.as_str().parse::<i32>().ok());

            let roll_behaviour: RollBehaviour;
            let modifier: Option<i32>;

            if let Some(x) = fatal_mod {
                // Fatal and reroll mechanics are mutually exclusive, so if a fatal case is found parse that...
                roll_behaviour = RollBehaviour::Fatal;
                modifier = Some(x);
            } else {
                // Otherwise test for reroll mechanics
                roll_behaviour = match adv_mod {
                    "AA" => RollBehaviour::DoubleAdvantage,
                    "A" => RollBehaviour::Advantage,
                    "D" => RollBehaviour::Disadvantage,
                    _ => RollBehaviour::Standard,
                };
                modifier = None;
            }

            for _ in 0..n_dice {
                let dice_collection = DiceBuilder::new()
                    .set_roll_max(die_size)
                    .set_roll_behaviour(roll_behaviour, modifier)
                    .build();

                dice_vector.push(dice_collection);
            }
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
    /// // Typical D&D dice notation
    /// let mut mod_collection: Vec<StaticModifier> = Vec::new();
    /// parse_static_elements(&mut mod_collection, "1d8+5");
    ///
    /// // D&D damage roll from the above, with a +1 weapon
    /// parse_static_elements(&mut mod_collection, "1d8+6");
    /// // or
    /// parse_static_elements(&mut mod_collection, "1d8+5+1");
    /// ```
    fn parse_static_elements(
        mod_vector: &mut Vec<StaticModifier>,
        notation: &str,
        rule_set: &Ruleset,
    ) {
        // Use a lazy wrapper so that the expression is only compiled a single time.
        // This expression should only be called once, but this is easy future proofing.
        static RE_STATIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\+\d+)|(-\d+)").unwrap());

        for capture in RE_STATIC.find_iter(notation) {
            let modifier = capture.as_str().parse::<i32>().unwrap();
            let behaviour = match rule_set {
                Ruleset::PF2e => ModifierBehaviour::CanCritical,
                Ruleset::DND5e => ModifierBehaviour::OnHit,
            };

            mod_vector.push(StaticModifier::new(modifier, behaviour));
        }
    }

    /// Take a pair of input strings from the user and parse into the elements representing the roll
    ///
    /// Breaks and iterates over comma separation for dice elements, then identifiers and
    /// captures all static modifiers, identified through their +/- prefix.
    ///
    /// # Examples
    /// ```
    /// let roll_collection = parse_user_input("1d6,2d4+5", Ruleset::DND5e);
    /// ```
    pub fn parse_user_input(notation: &str, rule_mode: Ruleset) -> RollCollection {
        let mut dice_vector: Vec<Dice> = Vec::new();

        for notation_fragment in notation.split(",") {
            RollCollection::parse_die_elements(&mut dice_vector, notation_fragment);
        }

        let mut mod_vector: Vec<StaticModifier> = Vec::new();
        RollCollection::parse_static_elements(&mut mod_vector, notation, &rule_mode);

        RollCollection::new(dice_vector, mod_vector, rule_mode)
    }

    /// Perform a turn roll against a specified armour class
    ///
    /// Returns the hit result as either miss, hit, or critical hit. Critical hits are
    /// determined using the internal rule specified by the struct instance. Internally
    /// this is just a call to either the RollCollection::eval_ac_roll_dnd() or
    /// RollCollection::eval_ac_roll_pathfinder() function.
    ///
    /// # Examples
    /// ```
    /// // Rolling 1d20+5
    /// let mut roll_collection = RollCollection::new(
    ///     vec![DiceBuilder::new().set_roll_max(20).build()],
    ///     vec![StaticModifier::new(5, EvalBehaviour::OnHit)],
    ///     Ruleset::PF2e
    /// );
    /// let target_ac = 15;
    /// let result = roll_collection.roll_against_armour_class(target_ac);
    /// ```
    pub fn roll_against_armour_class(&mut self, target_ac: i32) -> HitResult {
        match self.rule_mode {
            Ruleset::DND5e => self.eval_ac_roll_dnd(target_ac),
            Ruleset::PF2e => self.eval_ac_roll_pathfinder(target_ac),
        }
    }

    /// Roll the rollection as a damage roll with a specified hit outcome.
    ///
    /// Modules the damage according to the hit type (miss, hit, critical hit),
    /// the rule set used, and the behaviour of each element in the roll collection.
    ///
    /// CURRENTLY UNDER DEVELOPMENT, REQUIRE MORE IMPLEMENTATION IN THE DICE AND
    /// STATIC_MODIFIER STRUCTS TO ENABLE ALL BEHAVIOURS.
    ///
    /// # Examples
    /// ```
    /// // Roll damage for a 1d8+3 attack (hit)
    /// let mut roll_collection = RollCollection::new(
    ///     vec![DiceBuilder::new().set_roll_max(8).build()],
    ///     vec![StaticModifier::new(3, EvalBehaviour::OnHit)],
    ///     Ruleset::DND5e
    /// );
    /// let result = roll_collection.roll_damage_result(&HitResult::Hit);
    /// ```
    pub fn roll_damage_result(&mut self, hit_result: &HitResult) -> i32 {
        let dice_roll: i32 = self
            .dice
            .iter_mut()
            .map(|d| d.evaluate_result(Some(hit_result)))
            .sum();
        let static_mods: i32 = self
            .modifiers
            .iter()
            .map(|s| s.evaluate_result(Some(hit_result)))
            .sum();

        dice_roll + static_mods
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    // Useful for unit testing, don't need for real implementation
    impl Default for DiceCollection {
        fn default() -> Self {
            Self {
                n_die: 0,
                dice: Dice::new(0, None),
                roll_trait: ApplyTrait::Standard,
                rule_mode: Ruleset::DND5e,
            }
        }
    }

    fn unpack_roll_vector(roll_capture: &Vec<i32>) -> (i32, i32) {
        let obs_min: i32 = *roll_capture.iter().min().unwrap();
        let obs_max: i32 = *roll_capture.iter().max().unwrap();

        (obs_min, obs_max)
    }

    // Add eq tests once this issue is resolved...
    #[ignore]
    #[test]
    fn test_constructor() {
        let exp_dc = DiceCollection {
            n_die: 2,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::Standard,
            rule_mode: Ruleset::DND5e,
        };

        let obs_dc = DiceCollection::new(
            4,
            Dice::new(4, None),
            ApplyTrait::Standard,
            Ruleset::DND5e,
        );

        assert_eq!(exp_dc, obs_dc);
    }

    // region: DiceCollection::roll_sum tests

    #[test]
    fn test_roll_sum_0d6() {
        let mut dc = DiceCollection::default();

        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_sum(0, 6)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (0, 0));
    }

    #[test]
    fn test_roll_sum_1d1() {
        let mut dc = DiceCollection::default();
    
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_sum(1, 1)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 1));
    }

    #[test]
    fn test_roll_sum_1d6() {
        let mut dc = DiceCollection::default();

        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_sum(1, 6)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 6));
    }

    #[test]
    fn test_roll_sum_2d6() {
        let mut dc = DiceCollection::default();

        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_sum(2, 6)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (2, 12));
    }

    #[test]
    fn test_roll_sum_10d2() {
        let mut dc = DiceCollection::default();

        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_sum(10, 2)).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (10, 20));
    }

    // endregion:

    // region: DiceCollection::roll_standard tests

    #[test]
    fn test_roll_standard_on_critical_only() {
        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnCriticalOnly { doubles_with_crit: false },
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_standard();
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_standard_on_miss() {
        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_standard();
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_standard_on_standard() {
        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            ..DiceCollection::default()
        };

        // Range for 1d4 = (1, 4)
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_standard()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 4));
    }

    // endregion:

    // region: DiceCollection::roll_miss tests

    #[test]
    fn test_roll_miss_on_miss_only() {
        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_miss();
        assert!(obs_result > 0);
    }

    #[test]
    fn test_roll_miss_other() {
        // This may not be exhaustive as the ApplyTrait scope grows
        let trait_vector: Vec<ApplyTrait> = vec![
            ApplyTrait::Standard,
            ApplyTrait::Deadly{ extra_sides: 1, extra_dice: 1 },
            ApplyTrait::Fatal { upgraded_sides: 1 },
            ApplyTrait::OnCriticalOnly { doubles_with_crit: true },
        ];

        for roll_trait in trait_vector {
            let mut dc = DiceCollection {
                n_die: 1,
                dice: Dice::new(4, None),
                roll_trait: roll_trait,
                ..DiceCollection::default()
            };

            let obs_result = dc.roll_miss();
            assert_eq!(0, obs_result);
        }
    }

    // endregion:

    // region: DiceCollection::roll_critical tests

    #[test]
    fn test_roll_critical_on_critical_only_double() {

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnCriticalOnly { doubles_with_crit: true },
            ..DiceCollection::default()
        };

        // Range for 2 * 1d4 = (2, 8)
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (2, 8));
    }

    #[test]
    fn test_roll_critical_on_critical_only_single() {

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnCriticalOnly { doubles_with_crit: false },
            ..DiceCollection::default()
        };

        // Range for 1d4 = (1, 4)
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 4));
    }

    #[test]
    fn test_roll_critical_on_miss_only() {

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_critical();
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_critical_deadly() {

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::Deadly { extra_sides: 2, extra_dice: 1 },
            ..DiceCollection::default()
        };

        // Range for 2 * 1d4 + 1d2 = (3, 10)
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (3, 10));
    }

    #[test]
    fn test_roll_critical_fatal() {

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::Fatal { upgraded_sides: 6 },
            ..DiceCollection::default()
        };

        // Range for 3 * 1d6 = (3, 18)
        let roll_results: Vec<i32> = (0..10_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (3, 18));
    }

    #[test]
    fn test_roll_critical_standard() {
        // Test the case for standard just to show the internal execution is correct.

        let mut dc = DiceCollection {
            n_die: 1,
            dice: Dice::new(4, None),
            roll_trait: ApplyTrait::Standard,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_critical();
        assert!(obs_result >= 2);
        assert!(obs_result <= 8);
    }

    // endregion:

    /*

    // region: parse_die_elements() tests

    #[test]
    fn test_parse_die_elements_none() {
        let exp_result: Vec<Dice> = Vec::new();
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_elements_single() {
        let exp_result = vec![DiceBuilder::new().set_roll_max(8).build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d8+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_elements_multiple() {
        let exp_result = vec![
            DiceBuilder::new().set_roll_max(8).build(),
            DiceBuilder::new().set_roll_max(8).build(),
        ];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "2d8+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_elements_multidigit() {
        // Ensuring that the regex does not prematurely terminate dice values with 10s or 100s sizes.
        let exp_result = vec![
            DiceBuilder::new().set_roll_max(20).build(),
            DiceBuilder::new().set_roll_max(100).build(),
        ];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d20");
        RollCollection::parse_die_elements(&mut obs_result, "1d100+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_single_a() {
        let exp_result = vec![DiceBuilder::new()
            .set_roll_max(8)
            .set_roll_behaviour(RollBehaviour::Advantage, None)
            .build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d8A+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_double_a() {
        let exp_result = vec![DiceBuilder::new()
            .set_roll_max(8)
            .set_roll_behaviour(RollBehaviour::DoubleAdvantage, None)
            .build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d8AA+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_single_d() {
        let exp_result = vec![DiceBuilder::new()
            .set_roll_max(8)
            .set_roll_behaviour(RollBehaviour::Disadvantage, None)
            .build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d8D+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_single_invalid() {
        let exp_result = vec![DiceBuilder::new().set_roll_max(8).build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d8z+5");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_fatal() {
        let exp_result = vec![DiceBuilder::new()
            .set_roll_max(6)
            .set_roll_behaviour(RollBehaviour::Fatal, Some(10))
            .build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d6~10");
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_die_string_compete() {
        // Test a string with both advantage/disadvantage and fatal modifiers, to confirm the resolution priority
        let exp_result = vec![DiceBuilder::new()
            .set_roll_max(6)
            .set_roll_behaviour(RollBehaviour::Fatal, Some(10))
            .build()];
        let mut obs_result: Vec<Dice> = Vec::new();

        RollCollection::parse_die_elements(&mut obs_result, "1d6AA~10");
        assert_eq!(exp_result, obs_result);
    }

    // endregion:

    // region: parse_static_elements() tests

    #[test]
    fn test_parse_static_elements_single_pos() {
        let exp_result = vec![StaticModifier::new(5, ModifierBehaviour::OnHit)];
        let mut obs_result: Vec<StaticModifier> = Vec::new();

        RollCollection::parse_static_elements(&mut obs_result, "1d8+5", &Ruleset::DND5e);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_static_elements_single_neg() {
        let exp_result = vec![StaticModifier::new(-5, ModifierBehaviour::OnHit)];
        let mut obs_result: Vec<StaticModifier> = Vec::new();

        RollCollection::parse_static_elements(&mut obs_result, "1d8-5", &Ruleset::DND5e);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_static_elements_multiple() {
        let exp_result = vec![
            StaticModifier::new(5, ModifierBehaviour::OnHit),
            StaticModifier::new(-3, ModifierBehaviour::OnHit),
        ];
        let mut obs_result: Vec<StaticModifier> = Vec::new();

        RollCollection::parse_static_elements(&mut obs_result, "1d8+5-3", &Ruleset::DND5e);
        assert_eq!(exp_result, obs_result);
    }

    #[test]
    fn test_parse_static_elements_pf() {
        let exp_result = vec![StaticModifier::new(5, ModifierBehaviour::CanCritical)];
        let mut obs_result: Vec<StaticModifier> = Vec::new();

        RollCollection::parse_static_elements(&mut obs_result, "1d8+5", &Ruleset::PF2e);
        assert_eq!(exp_result, obs_result);
    }

    // endregion:

    // region: parse_user_input() tests

    #[test]
    fn test_parse_user_input() {
        // This function does not exhaustively test all dice string combinations, as these are tested
        // in the parsing functions.

        // 1d4
        let d1 = DiceBuilder::new().set_roll_max(4).build();

        // 2d6D
        let d2 = DiceBuilder::new()
            .set_roll_max(6)
            .set_roll_behaviour(RollBehaviour::Disadvantage, None)
            .build();
        let d3 = DiceBuilder::new()
            .set_roll_max(6)
            .set_roll_behaviour(RollBehaviour::Disadvantage, None)
            .build();

        // 1d6~12
        let d4 = DiceBuilder::new()
            .set_roll_max(6)
            .set_roll_behaviour(RollBehaviour::Fatal, Some(12))
            .build();

        let s_modfier = StaticModifier::new(7, ModifierBehaviour::OnHit);

        let exp_rc = RollCollection::new(vec![d1, d2, d3, d4], vec![s_modfier], Ruleset::DND5e);
        let obs_rc = RollCollection::parse_user_input("1d4,2d6D+7,1d6~12", Ruleset::DND5e);
        assert_eq!(exp_rc, obs_rc);
    }

    // endregion:
*/
}
