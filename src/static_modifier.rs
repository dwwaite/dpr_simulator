use crate::{HitResult, ModifierBehaviour};

/// A representation of the fixed damage element of an attack equation.
#[derive(Debug, PartialEq)]
pub struct StaticModifier {
    value: i32,
    mod_behaviour: ModifierBehaviour,
}

impl StaticModifier {
    /// Creates a new StaticModifier representation of a damage on an attack event.
    ///
    /// # Examples
    ///
    /// ```
    /// let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnHit);
    /// ```
    pub fn new(value: i32, mod_behaviour: ModifierBehaviour) -> StaticModifier {
        StaticModifier {
            value,
            mod_behaviour,
        }
    }

    /// Assess the value of the modifier for an instance of application.
    ///
    /// Accepts an optional hit result for modulating the return result based
    /// on the StaticModifier behaviour flag. If None is provided, the value
    /// is returned unmodified. The main use case for this is for instances
    /// associated with hit rolls, where as value modification is most likely
    /// to be required for damage rolls.
    ///
    /// # Examples
    /// ```
    /// let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnHit);
    ///
    /// let result = my_modifier.evaluate_result(None);
    /// let result = my_modifier.evaluate_result(Some(&HitResult::Miss));
    /// ```
    pub fn evaluate_result(&self, hit_condition: Option<&HitResult>) -> i32 {
        if let Some(hit_result) = hit_condition {
            match (&self.mod_behaviour, hit_result) {
                (&ModifierBehaviour::CanCritical, &HitResult::CriticalHit) => self.value * 2,
                (&ModifierBehaviour::CanCritical, &HitResult::Hit) => self.value,
                (&ModifierBehaviour::OnCritical, &HitResult::CriticalHit) => self.value,
                (&ModifierBehaviour::OnHit, &HitResult::Hit | &HitResult::CriticalHit) => {
                    self.value
                }
                (&ModifierBehaviour::OnMiss, _) => self.value,
                (_, _) => 0,
            }
        } else {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let exp_value = StaticModifier {
            value: -2,
            mod_behaviour: ModifierBehaviour::OnHit,
        };
        let obs_value = StaticModifier::new(-2, ModifierBehaviour::OnHit);

        assert_eq!(exp_value, obs_value);
    }

    #[test]
    fn test_evaluate_result_none() {
        let mod_options: Vec<ModifierBehaviour> = vec![
            ModifierBehaviour::CanCritical,
            ModifierBehaviour::OnCritical,
            ModifierBehaviour::OnHit,
            ModifierBehaviour::OnMiss,
        ];

        for mod_option in mod_options {
            let my_modifier = StaticModifier::new(5, mod_option);
            let obs_value = my_modifier.evaluate_result(None);
            assert_eq!(5, obs_value);
        }
    }

    // region: `evaluate_result()` ModifierBehaviour::OnHit tests

    #[test]
    fn test_evaluate_result_onhit_hit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnHit);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Hit));

        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_onhit_crit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnHit);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::CriticalHit));

        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_onhit_miss() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnHit);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Miss));

        assert_eq!(0, obs_value);
    }

    // endregion:

    // region: `evaluate_result()` ModifierBehaviour::OnCritical tests

    #[test]
    fn test_evaluate_result_oncrit_crit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnCritical);

        let obs_value = my_modifier.evaluate_result(Some(&HitResult::CriticalHit));
        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_oncrit_hit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnCritical);

        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Hit));
        assert_eq!(0, obs_value);
    }

    #[test]
    fn test_evaluate_result_oncrit_miss() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnCritical);

        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Miss));
        assert_eq!(0, obs_value);
    }

    // endregion:

    // region: `evaluate_result()` ModifierBehaviour::CanCritical tests

    #[test]
    fn test_evaluate_result_cancrit_crit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::CanCritical);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::CriticalHit));

        assert_eq!(10, obs_value);
    }

    #[test]
    fn test_evaluate_result_cancrit_hit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::CanCritical);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Hit));

        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_cancrit_miss() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::CanCritical);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Miss));

        assert_eq!(0, obs_value);
    }

    // endregion:

    // region: `evaluate_result()` ModifierBehaviour::OnMiss tests

    #[test]
    fn test_evaluate_result_onmiss_crit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnMiss);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::CriticalHit));

        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_onmiss_hit() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnMiss);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Hit));

        assert_eq!(5, obs_value);
    }

    #[test]
    fn test_evaluate_result_onmiss_miss() {
        let my_modifier = StaticModifier::new(5, ModifierBehaviour::OnMiss);
        let obs_value = my_modifier.evaluate_result(Some(&HitResult::Miss));

        assert_eq!(5, obs_value);
    }

    // endregion:
}
