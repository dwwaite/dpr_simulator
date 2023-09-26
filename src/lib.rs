use polars::prelude::*;
use simple_error::bail;
use std::error::Error;
use std::fs::File;

mod attackprofile;
use attackprofile::AttackProfile;
mod damageelement;
use damageelement::DamageElement;

//region Private functions

fn bundle_inputs(
    ac_targets: Vec<i32>,
    to_hit: i32,
    mainhand_attacks: i32,
    mainhand_notation: String,
    offhand_attack: i32,
    offhand_string: String,
) -> Vec<AttackProfile> {
    // Create a vector of AttackProfile structs corresponding to a vector of AC values.

    let profile_vector: Vec<AttackProfile> = ac_targets
        .into_iter()
        .map(|ac| {
            AttackProfile::new(
                ac,
                mainhand_attacks,
                offhand_attack,
                to_hit,
                DamageElement::from_notation_string(&mainhand_notation),
                DamageElement::from_notation_string(&offhand_string),
            )
        })
        .collect();

    profile_vector
}

fn process_ac_iteration(
    mut attack_profile: AttackProfile,
    number_turns: i32,
) -> Result<DataFrame, Box<dyn Error>> {
    // Simulate a specified number of attack iterations and format the results as a DataFrame.

    let iteration_counter: Vec<i32> = (0..number_turns).map(|x| x + 1).collect();
    let ac_record: Vec<i32> = vec![attack_profile.target_ac; number_turns as usize];

    let mut crit_counter: Vec<i32> = Vec::new();
    let mut damage_counter: Vec<i32> = Vec::new();

    for _ in 0..number_turns {
        let (n_crits, damage_rolled) = attack_profile.roll_turn();
        crit_counter.push(n_crits);
        damage_counter.push(damage_rolled);
    }

    // Bundle results into a DataFrame and return
    let results_df = match df!(
        "Iteration" => &iteration_counter,
        "Target_AC" => &ac_record,
        "Number_crits" => &crit_counter,
        "Total_damage" => &damage_counter
    ) {
        Ok(df) => df,
        _ => bail!(format!(
            "Unable to produce results matrix for AC value '{}'!",
            attack_profile.target_ac
        )),
    };
    Ok(results_df)
}

fn gather_dataframes(attack_results: Vec<LazyFrame>) -> Result<DataFrame, Box<dyn Error>> {
    // Compress the results into the final DataFrame for return

    // Documentation on the circumstances that cause the polars concat() function is lacking.
    // For now just return into the erorr box until I get a sighting of an error.
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
    to_hit: i32,
    mainhand_attacks: i32,
    mainhand_notation: String,
    offhand_attack: i32,
    offhand_string: String,
    number_turns: i32,
) -> Result<DataFrame, Box<dyn Error>> {
    // Partition the inputs over the range of AC values and simulate the attack turns.

    let profile_vector = bundle_inputs(
        ac_targets,
        to_hit,
        mainhand_attacks,
        mainhand_notation,
        offhand_attack,
        offhand_string,
    );

    // Populate a vector of results - later this will dispatch to multiple threads.
    let mut attack_results: Vec<LazyFrame> = Vec::new();
    for attack_profile in profile_vector {
        let ac_df = process_ac_iteration(attack_profile, number_turns)?;
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
    fn test_bundle_inputs_both() {
        // Test the turnsimulation.bundle_inputs() function with both weapons active.

        let mh_string = "1d4+1".to_string();
        let oh_string = "1d4".to_string();
        let mh_1 = DamageElement::from_notation_string(&mh_string);
        let mh_2 = DamageElement::from_notation_string(&mh_string);
        let oh_1 = DamageElement::from_notation_string(&oh_string);
        let oh_2 = DamageElement::from_notation_string(&oh_string);

        let exp_values: Vec<AttackProfile> = vec![
            AttackProfile::new(1, 1, 1, 1, mh_1, oh_1),
            AttackProfile::new(2, 1, 1, 1, mh_2, oh_2),
        ];

        let obs_values = bundle_inputs(vec![1, 2], 1, 1, mh_string, 1, oh_string);
        assert_eq!(exp_values, obs_values);
    }

    #[test]
    fn test_process_ac_iteration() {
        // Test the turnsimulation.process_ac_iteration() function for a success case.

        let mh_element = DamageElement::from_notation_string("1d4+1");
        let oh_element = DamageElement::from_notation_string("");
        let attack_profile = AttackProfile::new(1, 1, 0, 1, mh_element, oh_element);
        let number_turns = 5;

        let obs_result = process_ac_iteration(attack_profile, number_turns);
        assert!(obs_result.is_ok());

        // Test over the two predictable columns.
        // Ignore for crit and damage counters, as these just capture the output of functions tested in attackprofile.rs
        let obs_output = obs_result.unwrap();
        assert_eq!((5, 4), obs_output.shape());
        assert_eq!(
            &Series::new("Iteration", &[1, 2, 3, 4, 5]),
            obs_output.column("Iteration").unwrap()
        );
        assert_eq!(
            &Series::new("Target_AC", &[1, 1, 1, 1, 1]),
            obs_output.column("Target_AC").unwrap()
        );
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
        let mh_notation = "1d4+1".to_string();
        let oh_notation = "".to_string();

        let obs_result = process_simulation(vec![1, 2, 3], 10, 1, mh_notation, 0, oh_notation, 2);
        assert!(obs_result.is_ok());

        // Test the predictable output columns - expecting 3 (AC) repeats of 2 turns.
        let obs_output = obs_result.unwrap();
        assert_eq!((6, 4), obs_output.shape());
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
