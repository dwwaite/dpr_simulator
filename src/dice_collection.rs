use crate::dice::Dice;
use crate::{ApplyTrait, RollKind, Ruleset};

/// A representation of a collection of dice subject to roll conditions
#[derive(Debug)]
pub struct DiceCollection {
    pub n_die: i32,
    pub dice: Dice,
    roll_trait: ApplyTrait,
    rule_set: Ruleset,
}

// Implement PartialEq to invoke the Dice PartialEq.
impl PartialEq for DiceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.n_die == other.n_die
            //&& self.dice == other.dice
            && self.roll_trait == other.roll_trait
            && self.rule_set == other.rule_set
    }
}

// Useful for unit testing, don't need for real implementation
impl Default for DiceCollection {
    fn default() -> Self {
        Self {
            n_die: 1,
            dice: Dice::new(20),
            roll_trait: ApplyTrait::Standard,
            rule_set: Ruleset::DND5e,
        }
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
            rule_set: rule_set,
        }
    }

    /// A helper function to smooth over rolling branches when rolling criticals
    fn roll_sum(&mut self, n_dice: i32, n_sides: i32) -> i32 {
        (0..n_dice).map(|_| self.dice.roll(n_sides)).sum()
    }

    fn roll_standard(&mut self) -> i32 {
        // Catch cases where a regular would be ignored, otherwise return a standard roll
        match self.roll_trait {
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: _,
            } => 0,
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
                if doubles_with_crit {
                    self.roll_sum(self.n_die * 2, self.dice.sides)
                } else {
                    self.roll_sum(self.n_die, self.dice.sides)
                }
            }
            ApplyTrait::OnMissOnly => 0,
            // Pathfinder critical traits
            ApplyTrait::Deadly { extra_sides } => {
                //https://2e.aonprd.com/Traits.aspx?ID=570
                self.roll_sum(self.n_die * 2, self.dice.sides)
                    + self.roll_sum(self.n_die, extra_sides)
            }
            ApplyTrait::Fatal { upgraded_sides } => {
                //https://2e.aonprd.com/Traits.aspx?ID=597
                self.roll_sum((self.n_die * 2) + 1, upgraded_sides)
            }
            _ => self.roll_sum(self.n_die * 2, self.dice.sides),
        }
    }

    pub fn roll(&mut self, kind: RollKind) -> i32 {
        match kind {
            RollKind::Normal => self.roll_standard(),
            RollKind::Critical => self.roll_critical(),
            RollKind::Miss => self.roll_miss(),
        }
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

    // Add eq tests once this issue is resolved...
    #[ignore]
    #[test]
    fn test_constructor() {
        let exp_dc = DiceCollection {
            n_die: 2,
            dice: Dice::new(4),
            roll_trait: ApplyTrait::Standard,
            rule_set: Ruleset::DND5e,
        };

        let obs_dc = DiceCollection::new(4, Dice::new(4), ApplyTrait::Standard, Ruleset::DND5e);

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
    fn test_roll_standard_normal() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            ..DiceCollection::default()
        };

        // Range for 1d4 = (1, 4)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_standard()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 4));
    }

    #[test]
    fn test_roll_standard_on_critical_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_standard();
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_standard_on_miss_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_standard();
        assert_eq!(0, obs_result);
    }

    // endregion:

    // region: DiceCollection::roll_miss tests

    #[test]
    fn test_roll_miss_on_miss_only() {
        let mut dc = DiceCollection {
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
            ApplyTrait::Deadly { extra_sides: 1 },
            ApplyTrait::Fatal { upgraded_sides: 1 },
            ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
        ];

        for roll_trait in trait_vector {
            let mut dc = DiceCollection {
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
    fn test_roll_critical_normal() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            ..DiceCollection::default()
        };

        // Range for 2 * 1d4 = (2, 8)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (2, 8));
    }

    #[test]
    fn test_roll_critical_on_critical_only_double() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..DiceCollection::default()
        };

        // Range for 2 * 1d4 = (2, 8)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (2, 8));
    }

    #[test]
    fn test_roll_critical_on_critical_only_single() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..DiceCollection::default()
        };

        // Range for 1d4 = (1, 4)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (1, 4));
    }

    #[test]
    fn test_roll_critical_on_miss_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll_critical();
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_critical_deadly() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            roll_trait: ApplyTrait::Deadly { extra_sides: 2 },
            ..DiceCollection::default()
        };

        // Range for 2 * 1d4 + 1d2 = (3, 10)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (3, 10));
    }

    #[test]
    fn test_roll_critical_fatal() {
        let mut dc = DiceCollection {
            dice: Dice::new(4),
            roll_trait: ApplyTrait::Fatal { upgraded_sides: 6 },
            ..DiceCollection::default()
        };

        // Range for 3 * 1d6 = (3, 18)
        let roll_results: Vec<i32> = (0..1_000).map(|_| dc.roll_critical()).collect();
        let obs_results: (i32, i32) = unpack_roll_vector(&roll_results);

        assert_eq!(obs_results, (3, 18));
    }

    // endregion:

    // region: DiceCollection::roll tests

    #[test]
    fn test_roll_case_normal_standard() {
        let mut dc = DiceCollection {
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Normal);
        assert!(obs_result >= 1);
        assert!(obs_result <= 20);
    }

    #[test]
    fn test_roll_case_normal_on_critical_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Normal);
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_case_normal_on_miss_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Normal);
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_case_critical_standard() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::Standard,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Critical);
        assert!(obs_result >= 2);
    }

    #[test]
    fn test_roll_case_critical_on_critical_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Critical);
        assert!(obs_result >= 1);
    }

    #[test]
    fn test_roll_case_critical_on_miss_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Critical);
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_case_miss_standard() {
        let mut dc = DiceCollection::default();

        let obs_result = dc.roll(RollKind::Miss);
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_case_miss_on_critical_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Miss);
        assert_eq!(0, obs_result);
    }

    #[test]
    fn test_roll_case_miss_on_miss_only() {
        let mut dc = DiceCollection {
            roll_trait: ApplyTrait::OnMissOnly,
            ..DiceCollection::default()
        };

        let obs_result = dc.roll(RollKind::Miss);
        assert!(obs_result > 0);
    }

    // endregion:
}
