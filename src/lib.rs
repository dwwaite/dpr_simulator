use polars::prelude::*;
use simple_error::bail;
use std::error::Error;
use std::fs::File;

mod attackprofile;
use attackprofile::AttackProfile;
mod damageelement;
use damageelement::DamageElement;
mod dice;

#[derive(Debug, PartialEq)]
pub enum HitResult {
    Miss,
    Hit,
    CriticalHit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ruleset {
    DND5e,
    PF2e,
}

//region Private functions

fn bundle_inputs(
    ac_targets: Vec<i32>,
    to_hit: Vec<i32>,
    weapon_details: Vec<String>,
    ruleset: Ruleset,
) -> Vec<AttackProfile> {
    // Create a vector of AttackProfile structs corresponding to a vector of AC values.

    let profile_vector: Vec<AttackProfile> = ac_targets
        .into_iter()
        .map(|ac| {
            // If struct was changed could create this once and borrow in AttackProfile
            let damage_elements: Vec<DamageElement> = to_hit
                .iter()
                .zip(&weapon_details)
                .map(|(h, d)| DamageElement::from_notation_string(*h, d))
                .collect();

            AttackProfile::new(ac, damage_elements, ruleset)
        })
        .collect();

    profile_vector
}

fn bundle_results(
    ac_value: i32,
    crit_counter: Vec<i32>,
    hit_counter: Vec<i32>,
    damage_counter: Vec<i32>,
) -> DataFrame {
    // Collect the results of an attack profile simulation into a polars DataFrame

    // Compute the number of iterations, and replicate the AC over the size of the input vectors
    let max_len: i32 = (crit_counter.len() as i32) + 1;
    let iteration_counter: Vec<i32> = (1..max_len).collect();
    let ac_counter: Vec<i32> = vec![ac_value; crit_counter.len()];

    // Create the DataFrame. THis function cannot fail ni this scope, so just unwrap and return.
    df!(
        "Iteration" => &iteration_counter,
        "Target_AC" => &ac_counter,
        "Number_hits" => &hit_counter,
        "Number_crits" => &crit_counter,
        "Total_damage" => &damage_counter
    )
    .unwrap()
}

fn evaluate_attack_profile(attack_profile: AttackProfile, number_turns: i32) -> DataFrame {
    // Simulate a specified number of attack iterations and format the results as a DataFrame.

    //roll_turn(&self, roll_element: &mut ThreadRng) -> (crit_counter, hit_counter, total_damage)
    let mut crit_counter: Vec<i32> = Vec::new();
    let mut hit_counter: Vec<i32> = Vec::new();
    let mut damage_counter: Vec<i32> = Vec::new();

    let mut roll_element = rand::thread_rng();

    for _ in 0..number_turns {
        let (n_crits, n_hits, damage_rolled) = attack_profile.roll_turn(&mut roll_element);
        crit_counter.push(n_crits);
        hit_counter.push(n_hits);
        damage_counter.push(damage_rolled);
    }

    // Bundle results into a DataFrame and return
    bundle_results(
        attack_profile.target_ac,
        crit_counter,
        hit_counter,
        damage_counter,
    )
}

fn gather_dataframes(attack_results: Vec<LazyFrame>) -> Result<DataFrame, Box<dyn Error>> {
    // Compress the results into the final DataFrame for return

    // Documentation on the circumstances that cause the polars concat() function is lacking.
    // For now just return into the error box until I get a sighting of an error.
    let concat_result = concat(attack_results, true, true)?;

    let final_df = match concat_result.collect() {
        Ok(df) => df,
        _ => bail!("Error collecting individual jobs into single data frame!"),
    };

    Ok(final_df)
}

//endregion

//region Public functions

pub fn process_simulation(
    ac_targets: Vec<i32>,
    to_hit: Vec<i32>,
    weapon_details: Vec<String>,
    ruleset: Ruleset,
    number_turns: i32,
) -> Result<DataFrame, Box<dyn Error>> {
    // Partition the inputs over the range of AC values and simulate the attack turns.

    let profile_vector = bundle_inputs(ac_targets, to_hit, weapon_details, ruleset);

    // Populate a vector of results - later this will dispatch to multiple threads.
    let mut attack_results: Vec<LazyFrame> = Vec::new();
    for attack_profile in profile_vector {
        let ac_df = evaluate_attack_profile(attack_profile, number_turns);
        attack_results.push(ac_df.lazy());
    }

    gather_dataframes(attack_results)
}

pub fn write_to_parquet(
    output_path: &str,
    mut file_content: DataFrame,
) -> Result<(), Box<dyn Error>> {
    // Write the DataFrame content into the compressed parquet format.

    let error_msg = format!("Unable to write to output file path '{}'!", output_path);

    // Create the file path, and premature return if it fails
    let create_result = File::create(output_path);
    if create_result.is_err() {
        bail!(error_msg);
    }

    // If we get this far, no error was encountered.
    let target_file = create_result.unwrap();
    match ParquetWriter::new(target_file).finish(&mut file_content) {
        Ok(_) => (),
        _ => bail!(error_msg),
    };

    Ok(())
}

//endregion

//region Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_bundle_inputs() {
        // Test the function in a single call over a set of inputs.

        // Set the input values
        let input_acs = vec![1, 2];
        let input_hits = vec![3, 4];
        let input_details = vec!["1d4".to_string(), "1d6".to_string()];

        // Set the expected output vector
        let mut exp_profiles: Vec<AttackProfile> = Vec::new();
        exp_profiles.push(AttackProfile::new(
            1,
            vec![
                DamageElement::from_notation_string(3, "1d4"),
                DamageElement::from_notation_string(4, "1d6"),
            ],
            Ruleset::DND5e,
        ));
        exp_profiles.push(AttackProfile::new(
            2,
            vec![
                DamageElement::from_notation_string(3, "1d4"),
                DamageElement::from_notation_string(4, "1d6"),
            ],
            Ruleset::DND5e,
        ));

        let obs_profiles = bundle_inputs(input_acs, input_hits, input_details, Ruleset::DND5e);
        assert_eq!(exp_profiles, obs_profiles);
    }

    #[test]
    fn test_bundle_results() {
        // Test the behaviour of the bundle_results() function, assuming no errors.

        let input_ac = 5;
        let input_crits = vec![0, 1, 2, 3, 4];
        let input_hits = vec![2, 4, 6, 8, 10];
        let input_damage = vec![10, 12, 14, 16, 18];

        let exp_iteration = Series::new("Iteration", &vec![1, 2, 3, 4, 5]);
        let exp_ac = Series::new("Target_AC", &vec![5; 5]);
        let exp_crits = Series::new("Number_crits", &input_crits);
        let exp_hits = Series::new("Number_hits", &input_hits);
        let exp_damage = Series::new("Total_damage", &input_damage);

        let obs_df = bundle_results(input_ac, input_crits, input_hits, input_damage);

        // Check the shape and contents
        assert_eq!((5, 5), obs_df.shape());
        assert_eq!(&exp_iteration, obs_df.column("Iteration").unwrap());
        assert_eq!(&exp_ac, obs_df.column("Target_AC").unwrap());
        assert_eq!(&exp_crits, obs_df.column("Number_crits").unwrap());
        assert_eq!(&exp_hits, obs_df.column("Number_hits").unwrap());
        assert_eq!(&exp_damage, obs_df.column("Total_damage").unwrap());
    }

    #[test]
    fn test_gather_dataframes() {
        // Test the turnsimulation.gather_dataframes() function for a success case.

        let first_segment = df!(
            "Numbers" => &[1, 2, 3],
            "Letters" => &["a", "b", "c"],
        )
        .unwrap()
        .lazy();
        let second_segment = df!(
            "Numbers" => &[4, 5, 6],
            "Letters" => &["d", "e", "f"],
        )
        .unwrap()
        .lazy();

        let obs_result = gather_dataframes(vec![first_segment, second_segment]);
        assert!(obs_result.is_ok());

        // Test the contents of the returned data frame
        let obs_output = obs_result.unwrap();
        assert_eq!((6, 2), obs_output.shape());
        assert_eq!(
            &Series::new("Numbers", &[1, 2, 3, 4, 5, 6]),
            obs_output.column("Numbers").unwrap()
        );
        assert_eq!(
            &Series::new("Letters", &["a", "b", "c", "d", "e", "f"]),
            obs_output.column("Letters").unwrap()
        );
    }

    #[test]
    fn test_gather_dataframes_fail() {
        // Test the turnsimulation.gather_dataframes() function for a failure case.

        let first_segment = df!(
            "Numbers" => &[1, 2, 3],
        )
        .unwrap()
        .lazy();
        let second_segment = df!(
            "Letters" => &["d", "e", "f"],
        )
        .unwrap()
        .lazy();

        let obs_result = gather_dataframes(vec![first_segment, second_segment]);
        assert!(obs_result.is_err());
        assert_eq!(
            "Error collecting individual jobs into single data frame!",
            obs_result.unwrap_err().to_string()
        );
    }

    #[test]
    fn test_process_simulation() {
        /* Test the complete run of the turnsimulation.process_simulation() function.
           Only testing over the success case, as internal errors are tested in each subordinate function, and
            this just propagates those errors.
        */

        let obs_result = process_simulation(
            vec![1, 2, 3],
            vec![10],
            vec!["1d4+1".to_string()],
            Ruleset::DND5e,
            2,
        );

        // Test the predictable output columns - expecting 3 (AC) repeats of 2 turns.
        assert!(obs_result.is_ok());
        let obs_output = obs_result.unwrap();

        assert_eq!((6, 5), obs_output.shape());
        assert_eq!(
            &Series::new("Iteration", &[1, 2, 1, 2, 1, 2]),
            obs_output.column("Iteration").unwrap()
        );
        assert_eq!(
            &Series::new("Target_AC", &[1, 1, 2, 2, 3, 3]),
            obs_output.column("Target_AC").unwrap()
        );
    }

    #[test]
    fn test_write_to_parquet() {
        // Test the write_to_parquet() function for the success case.

        let file_path = "test_write_to_parquet.txt";
        let df = df!("Temp" => &[1, 2, 3]).unwrap();

        let obs_result = write_to_parquet(file_path, df);

        // Test the return values and that the file exists
        assert!(obs_result.is_ok());
        assert_eq!((), obs_result.unwrap());
        assert!(Path::new(file_path).exists());

        // Tidy up
        let _ = fs::remove_file(file_path);
    }

    #[test]
    fn test_write_to_parquet_fail() {
        // Test the write_to_parquet() function when writing fails (via an invalid path).

        let file_path = "bad_path/test_write_to_parquet.txt";
        let df = df!("Temp" => &[1, 2, 3]).unwrap();

        let obs_result = write_to_parquet(file_path, df);

        // Test the return values
        assert!(obs_result.is_err());
        assert_eq!(
            "Unable to write to output file path 'bad_path/test_write_to_parquet.txt'!",
            obs_result.unwrap_err().to_string()
        );
    }
}

//endregion
