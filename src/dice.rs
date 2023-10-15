use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Die {
    roll_min: i32,
    roll_max: i32,
}

impl Die {
    pub fn new(roll_min: i32, roll_max: i32) -> Die {
        Die {
            roll_min: roll_min,
            roll_max: roll_max,
        }
    }

    pub fn roll(&self, roll_element: &mut ThreadRng) -> i32 {
        // Roll the outcome for the Die element.

        roll_element.gen_range(self.roll_min..self.roll_max + 1)
    }
}

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll() {
        // Test the Die::roll() function.
        // There's an unlike-but-not-zero chance that this will fail to see the maximum value due to bad luck.

        let min_value = 1;
        let max_value = 6;
        let mut roll_element = rand::thread_rng();
        let my_die = Die::new(min_value, max_value);

        let mut roll_results: Vec<i32> = Vec::new();
        for _ in 0..10000 {
            roll_results.push(my_die.roll(&mut roll_element));
        }

        // Assess the results
        let obs_min: i32 = *roll_results.iter().min().unwrap();
        let obs_max: i32 = *roll_results.iter().max().unwrap();

        assert_eq!(min_value, obs_min);
        assert_eq!(max_value, obs_max);
    }
}

//endregion
