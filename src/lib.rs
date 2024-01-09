use dicecontext::DiceContext;
use polars::prelude::*;
use simple_error::bail;
use std::error::Error;
use std::fs::File;

mod attackprofile;
use attackprofile::AttackProfile;
mod dice;
mod dicecontext;

#[derive(Debug, PartialEq)]
pub enum HitResult {
    Miss,
    Hit,
    CriticalHit,
}

#[derive(Debug, PartialEq)]
pub enum Reroll {
    Standard,
    Advantage,
    DoubleAdvantage,
    Disadvantage,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ruleset {
    DND5e,
    PF2e,
}

//region Private functions

fn resize_vector(base_vector: &mut Vec<String>, new_value: String, iterations: usize) {
    // Extend the length of the input by appending the specified value a given number of times.

    let new_vector = vec![new_value; iterations];
    base_vector.extend(new_vector);
}

fn produce_attackprofile(
    target_ac: i32,
    hit_details: &Vec<String>,
    weapon_details: &Vec<String>,
    ruleset: &Ruleset,
) -> AttackProfile {
    // Create a vector of AttackProfile structs corresponding to a vector of AC values.

    let hit_context = hit_details
        .into_iter()
        .map(|s| DiceContext::parse_dice_string(s))
        .collect();

    let weapon_context = weapon_details
        .into_iter()
        .map(|s| DiceContext::parse_dice_string(s))
        .collect();

    AttackProfile::new(target_ac, hit_context, weapon_context, *ruleset)
}

fn map_profiles_to_ac(
    ac_targets: Vec<i32>,
    hit_details: Vec<String>,
    weapon_details: Vec<String>,
    ruleset: Ruleset,
) -> Vec<AttackProfile> {
    // Create a vector of AttackProfile structs corresponding to a vector of AC values.

    let profile_vector: Vec<AttackProfile> = ac_targets
        .into_iter()
        .map(|i| produce_attackprofile(i, &hit_details, &weapon_details, &ruleset))
        .collect();

    profile_vector
}

fn results_to_dataframe(
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
    results_to_dataframe(
        attack_profile.target_ac,
        crit_counter,
        hit_counter,
        damage_counter,
    )
}

//endregion

//region Public functions

pub fn equalise_input_vectors(first_vector: &mut Vec<String>, second_vector: &mut Vec<String>) {
    // Compare the lengths of two input vectors and replicate the last value of the shorter so that
    //  the lengths are equal.

    if first_vector.len() == second_vector.len() {
        // No-op if lengths are equal
        ()
    } else if first_vector.len() > second_vector.len() {
        // Case if the first vector is longer than the second
        let length_diff = first_vector.len() - second_vector.len();
        let last_value = second_vector.last().unwrap().to_string();
        resize_vector(second_vector, last_value, length_diff);
    } else if first_vector.len() < second_vector.len() {
        // Case if the second vector is longer than the first
        let length_diff = second_vector.len() - first_vector.len();
        let last_value = first_vector.last().unwrap().to_string();
        resize_vector(first_vector, last_value, length_diff);
    }
}

pub fn process_simulation(
    ac_targets: Vec<i32>,
    hit_details: Vec<String>,
    weapon_details: Vec<String>,
    ruleset: Ruleset,
    number_turns: i32,
) -> DataFrame {
    // Partition the inputs over the range of AC values and simulate the attack turns.

    let profile_vector: Vec<AttackProfile> =
        map_profiles_to_ac(ac_targets, hit_details, weapon_details, ruleset);

    let attack_results: Vec<LazyFrame> = profile_vector
        .into_iter()
        .map(|ap| evaluate_attack_profile(ap, number_turns).lazy())
        .collect();

    // Documentation on the circumstances that cause the polars concat() function is lacking.
    // For now just unwrap and return until I get a sighting of an error, at which point a
    //  separate function might be required.
    concat(attack_results, true, true)
        .unwrap()
        .collect()
        .unwrap()
}

pub fn write_to_parquet(
    output_path: &str,
    file_content: &mut DataFrame,
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
    match ParquetWriter::new(target_file).finish(file_content) {
        Ok(_) => (),
        _ => bail!(error_msg),
    };

    Ok(())
}

pub fn summarise_results(results_df: DataFrame) -> DataFrame {
    results_df
        .lazy()
        .groupby(["Target_AC"])
        .agg([
            col("Number_hits").mean().alias("Hits per round (mean)"),
            col("Number_crits")
                .mean()
                .alias("Critical hits per round (mean)"),
            col("Total_damage").mean().alias("Damage per round (mean)"),
        ])
        .sort("Target_AC", Default::default())
        .rename(["Target_AC"], ["Target AC"])
        .collect()
        .unwrap()
}

//endregion

//region Unit tests

#[cfg(test)]
mod tests {

    use crate::dice::Die;

    use super::*;
    use std::fs;
    use std::path::Path;

    fn create_string_vector(input_values: Vec<&str>) -> Vec<String> {
        // Convert a Vec<&str> to a Vec<String>
        input_values.iter().map(|x| x.to_string()).collect()
    }

    fn dataframes_are_equal(left_df: DataFrame, right_df: DataFrame) -> () {
        // Check the shape and column sequence
        assert_eq!(left_df.shape(), right_df.shape());
        assert_eq!(left_df.get_column_names(), right_df.get_column_names());

        // Move through the columns of the left DataFrame, and test that right-hand
        //  columns match.
        for column_name in &left_df.get_column_names() {
            assert_eq!(
                left_df.column(column_name).unwrap(),
                right_df.column(column_name).unwrap()
            );
        }
    }

    #[test]
    fn test_resize_vector() {
        // Test the function when a resize occurs.

        let exp_vector = create_string_vector(vec!["a", "b", "c", "d", "d"]);
        let mut input_vector = create_string_vector(vec!["a", "b", "c"]);

        resize_vector(&mut input_vector, "d".to_string(), 2);
        assert_eq!(exp_vector, input_vector);
    }

    #[test]
    fn test_resize_vector_no_change() {
        // Test the function when no resize is required.

        let exp_vector = create_string_vector(vec!["a", "b", "c"]);
        let mut input_vector = create_string_vector(vec!["a", "b", "c"]);

        resize_vector(&mut input_vector, "d".to_string(), 0);
        assert_eq!(exp_vector, input_vector);
    }

    #[test]
    fn test_produce_attackprofile_single() {
        // Test the function to produce for a single hit/weapon input.

        let exp_ap = AttackProfile::new(
            10,
            vec![DiceContext::parse_dice_string("1d4+1")],
            vec![DiceContext::parse_dice_string("1d10+1")],
            Ruleset::DND5e,
        );

        let obs_ap = produce_attackprofile(
            10,
            &vec!["1d4+1".to_string()],
            &vec!["1d10+1".to_string()],
            &Ruleset::DND5e,
        );

        assert_eq!(exp_ap, obs_ap);
    }

    #[test]
    fn test_produce_attackprofile_multiple() {
        // Test the function to produce for multiple hit/weapon inputs.

        let exp_ap = AttackProfile::new(
            10,
            vec![
                DiceContext::parse_dice_string("1d4+1"),
                DiceContext::parse_dice_string("1d6+2"),
            ],
            vec![
                DiceContext::parse_dice_string("1d10+3"),
                DiceContext::parse_dice_string("1d12+4"),
            ],
            Ruleset::DND5e,
        );

        let obs_ap = produce_attackprofile(
            10,
            &vec!["1d4+1".to_string(), "1d6+2".to_string()],
            &vec!["1d10+3".to_string(), "1d12+4".to_string()],
            &Ruleset::DND5e,
        );

        assert_eq!(exp_ap, obs_ap);
    }

    #[test]
    fn test_map_profiles_to_ac() {
        // Test the ability to produce multiple AttackProfiles from a single set of input
        //  strings to produce the DiceContext structs.

        let exp_aps = vec![
            AttackProfile::new(
                10,
                vec![DiceContext::parse_dice_string("1d4+1")],
                vec![DiceContext::parse_dice_string("1d12+4")],
                Ruleset::DND5e,
            ),
            AttackProfile::new(
                15,
                vec![DiceContext::parse_dice_string("1d4+1")],
                vec![DiceContext::parse_dice_string("1d12+4")],
                Ruleset::DND5e,
            ),
        ];

        let obs_aps = map_profiles_to_ac(
            vec![10, 15],
            vec!["1d4+1".to_string()],
            vec!["1d12+4".to_string()],
            Ruleset::DND5e,
        );

        assert_eq!(exp_aps, obs_aps);
    }

    #[test]
    fn test_results_to_dataframe() {
        // Test the behaviour of the results_to_dataframe() function, assuming no errors.

        let input_ac = 5;
        let input_crits = vec![0, 1, 2, 3, 4];
        let input_hits = vec![2, 4, 6, 8, 10];
        let input_damage = vec![10, 12, 14, 16, 18];

        let exp_df = df![
            "Iteration" => &vec![1, 2, 3, 4, 5],
            "Target_AC" => &vec![5; 5],
            "Number_hits" => &input_hits,
            "Number_crits" => &input_crits,
            "Total_damage" => &input_damage
        ]
        .unwrap();
        let obs_df = results_to_dataframe(input_ac, input_crits, input_hits, input_damage);

        dataframes_are_equal(exp_df, obs_df);
    }

    #[test]
    fn test_evaluate_attack_profile() {
        // Test the behaviour of the evaluate_attack_profile() function.
        // Using very carefully controlled dice to have a predictable output.

        let attackprofile = AttackProfile::new(
            1,
            vec![DiceContext::new(vec![Die::new(2, 3, Reroll::Standard)], 5)],
            vec![DiceContext::parse_dice_string("1d1+1")],
            Ruleset::DND5e,
        );

        let exp_df = df![
            "Iteration" => vec![1, 2, 3, 4, 5],
            "Target_AC" => vec![1; 5],
            "Number_hits" => vec![1; 5],
            "Number_crits" => vec![0; 5],
            "Total_damage" => vec![2; 5],
        ]
        .unwrap();

        let obs_df = evaluate_attack_profile(attackprofile, 5);
        dataframes_are_equal(exp_df, obs_df);
    }

    #[test]
    fn test_equalise_input_vectors_increase_first() {
        // Test the function when a the first vector needs to be resized.

        let mut first_vector = create_string_vector(vec!["a", "b"]);
        let mut second_vector = create_string_vector(vec!["A", "B", "C"]);

        // Test that the first vector is resized, and the second vector is unchanged.
        let exp_first = create_string_vector(vec!["a", "b", "b"]);
        let exp_second = create_string_vector(vec!["A", "B", "C"]);

        equalise_input_vectors(&mut first_vector, &mut second_vector);
        assert_eq!(exp_first, first_vector);
        assert_eq!(exp_second, second_vector);
    }

    #[test]
    fn test_equalise_input_vectors_increase_second() {
        // Test the function when a the first vector needs to be resized.

        let mut first_vector = create_string_vector(vec!["a", "b", "c", "d", "e"]);
        let mut second_vector = create_string_vector(vec!["A", "B", "C"]);

        // Test that the second vector is resized, and the first vector is unchanged.
        let exp_first = create_string_vector(vec!["a", "b", "c", "d", "e"]);
        let exp_second = create_string_vector(vec!["A", "B", "C", "C", "C"]);

        equalise_input_vectors(&mut first_vector, &mut second_vector);
        assert_eq!(exp_first, first_vector);
        assert_eq!(exp_second, second_vector);
    }

    #[test]
    fn test_equalise_input_vectors_unchanged() {
        // Test the function when no resize if required.

        let mut first_vector = create_string_vector(vec!["a", "b", "c"]);
        let mut second_vector = create_string_vector(vec!["A", "B", "C"]);

        // Test that the neither vector is resized.
        let exp_first = create_string_vector(vec!["a", "b", "c"]);
        let exp_second = create_string_vector(vec!["A", "B", "C"]);

        equalise_input_vectors(&mut first_vector, &mut second_vector);
        assert_eq!(exp_first, first_vector);
        assert_eq!(exp_second, second_vector);
    }

    #[test]
    fn test_process_simulation() {
        /* Test the complete run of the turnsimulation.process_simulation() function.
           Only testing over the success case, as internal behaviours are tested in relevant
            unit tests.
        */

        let exp_df = df![
            "Iteration" => vec![1, 2, 3, 4, 5, 1, 2, 3, 4, 5],
            "Target_AC" => vec![0, 0, 0, 0, 0, 10, 10, 10, 10, 10],
            "Number_hits" => vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0],
            "Number_crits" => vec![0; 10],
            "Total_damage" => vec![2, 2, 2, 2, 2, 0, 0, 0, 0, 0],
        ]
        .unwrap();

        let obs_df = process_simulation(
            vec![0, 10],
            vec!["1d1+1".to_string()],
            vec!["1d1+1".to_string()],
            Ruleset::DND5e,
            5,
        );
        dataframes_are_equal(exp_df, obs_df);
    }

    #[test]
    fn test_write_to_parquet() {
        // Test the write_to_parquet() function for the success case.

        let file_path = "test_write_to_parquet.txt";
        let mut df = df!("Temp" => &[1, 2, 3]).unwrap();

        let obs_result = write_to_parquet(file_path, &mut df);

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
        let mut df = df!("Temp" => &[1, 2, 3]).unwrap();

        let obs_result = write_to_parquet(file_path, &mut df);

        // Test the return values
        assert!(obs_result.is_err());
        assert_eq!(
            "Unable to write to output file path 'bad_path/test_write_to_parquet.txt'!",
            obs_result.unwrap_err().to_string()
        );
    }

    #[test]
    fn test_summarise_results() {
        // Test the behaviour of the summarise_results() function.

        let input_df = df![
            "Target_AC" => vec![0, 0, 0, 1, 1, 1, 2, 2, 2],
            "Number_hits" => vec![0, 1, 2, 4, 6, 8, 1, 2, 3],
            "Number_crits" => vec![0, 1, 2, 4, 6, 8, 1, 2, 3],
            "Total_damage" => vec![0, 1, 2, 4, 6, 8, 1, 2, 3],
        ]
        .unwrap();

        let exp_df = df![
            "Target AC" => vec![0, 1, 2],
            "Hits per round (mean)" => vec![1, 6, 2],
            "Critical hits per round (mean)" => vec![1, 6, 2],
            "Damage per round (mean)" => vec![1, 6, 2],
        ]
        .unwrap();

        let obs_df = summarise_results(input_df);
        dataframes_are_equal(exp_df, obs_df);
    }
}

//endregion
