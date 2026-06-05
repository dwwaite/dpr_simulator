#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModifierBehaviour {
    OnHit,
    OnMiss,
}

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
}
