use crate::{ApplyTrait, RollKind};

/// A representation of the fixed damage element of an attack equation.
#[derive(Debug, PartialEq)]
pub struct StaticModifier {
    value: i32,
    mod_trait: ApplyTrait,
}

impl StaticModifier {
    /// Creates a new StaticModifier representation of a damage on an attack event.
    ///
    /// # Examples
    ///
    /// ```
    /// let my_modifier = StaticModifier::new(5, ApplyTrait::Standard);
    /// ```
    pub fn new(value: i32, mod_trait: ApplyTrait) -> StaticModifier {
        StaticModifier { value, mod_trait }
    }

    fn roll_standard(&self) -> i32 {
        // Catch cases where a regular would be ignored, otherwise return a standard roll
        match self.mod_trait {
            ApplyTrait::OnCriticalOnly { doubles_with_crit } => 0,
            ApplyTrait::OnMissOnly => 0,
            _ => self.value,
        }
    }

    fn roll_miss(&self) -> i32 {
        // Catch case where roll occurs on miss, otherwise return 0
        match self.mod_trait {
            ApplyTrait::OnMissOnly => self.value,
            _ => 0,
        }
    }

    fn roll_critical(&self) -> i32 {
        match self.mod_trait {
            ApplyTrait::OnMissOnly => 0,
            ApplyTrait::OnCriticalOnly { doubles_with_crit } => {
                if doubles_with_crit {
                    self.value * 2
                } else {
                    self.value
                }
            }
            _ => self.value,
        }
    }

    pub fn roll(&self, kind: RollKind) -> i32 {
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

    // Useful for unit testing, don't need for real implementation
    impl Default for StaticModifier {
        fn default() -> Self {
            Self {
                value: 1,
                mod_trait: ApplyTrait::Standard,
            }
        }
    }

    #[test]
    fn test_constructor() {
        let exp_value = StaticModifier::default();
        let obs_value = StaticModifier::new(1, ApplyTrait::Standard);

        assert_eq!(exp_value, obs_value);
    }

    // region: StaticModifier::roll_standard tests

    #[test]
    fn test_roll_standard_normal() {
        let sm = StaticModifier::default();

        assert_eq!(1, sm.roll_standard());
    }

    #[test]
    fn test_roll_standard_normal_zero() {
        let sm = StaticModifier {
            value: 0,
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_standard());
    }

    #[test]
    fn test_roll_standard_on_critical_only_double() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_standard());
    }

    #[test]
    fn test_roll_standard_on_critical_only_single() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_standard());
    }

    #[test]
    fn test_roll_standard_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_standard());
    }

    // endregion:

    // region: StaticModifier::roll_miss()

    #[test]
    fn test_roll_miss_standard() {
        let sm = StaticModifier::default();

        assert_eq!(0, sm.roll_miss());
    }

    #[test]
    fn test_roll_miss_on_critical_only_double() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_miss());
    }

    #[test]
    fn test_roll_miss_on_critical_only_single() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_miss());
    }

    #[test]
    fn test_roll_miss_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(1, sm.roll_miss());
    }

    #[test]
    fn test_roll_miss_on_miss_only_zero() {
        let sm = StaticModifier::new(0, ApplyTrait::OnMissOnly);

        assert_eq!(0, sm.roll_miss());
    }

    // endregion:

    // region: StaticModifier::roll_critical tests

    #[test]
    fn test_roll_critical_standard() {
        let sm = StaticModifier::default();

        assert_eq!(1, sm.roll_critical());
    }

    #[test]
    fn test_roll_critical_on_critical_only_double() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(2, sm.roll_critical());
    }

    #[test]
    fn test_roll_critical_on_critical_only_single() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: false,
            },
            ..StaticModifier::default()
        };

        assert_eq!(1, sm.roll_critical());
    }

    #[test]
    fn test_roll_critical_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll_critical());
    }

    // endregion:

    // region: StaticModifier::roll tests

    #[test]
    fn test_roll_case_standard_normal() {
        let sm = StaticModifier::default();

        assert_eq!(1, sm.roll(RollKind::Normal));
    }

    #[test]
    fn test_roll_case_standard_on_critical_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll(RollKind::Normal));
    }

    #[test]
    fn test_roll_case_standard_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll(RollKind::Normal));
    }

    #[test]
    fn test_roll_case_miss_standard() {
        let sm = StaticModifier::default();

        assert_eq!(0, sm.roll(RollKind::Miss));
    }

    #[test]
    fn test_roll_case_miss_on_critical_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll(RollKind::Miss));
    }

    #[test]
    fn test_roll_case_miss_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(1, sm.roll(RollKind::Miss));
    }

    #[test]
    fn test_roll_case_critical_standard() {
        let sm = StaticModifier::default();

        assert_eq!(1, sm.roll(RollKind::Critical));
    }

    #[test]
    fn test_roll_case_critical_on_critical_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnCriticalOnly {
                doubles_with_crit: true,
            },
            ..StaticModifier::default()
        };

        assert_eq!(2, sm.roll(RollKind::Critical));
    }

    #[test]
    fn test_roll_case_critical_on_miss_only() {
        let sm = StaticModifier {
            mod_trait: ApplyTrait::OnMissOnly,
            ..StaticModifier::default()
        };

        assert_eq!(0, sm.roll(RollKind::Critical));
    }

    // endregion:
}
