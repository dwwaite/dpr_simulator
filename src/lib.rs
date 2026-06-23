use polars::prelude::*;
use rayon::prelude::*;
use simple_error::bail;
use std::{cmp::Ordering, error::Error, fs::File};

mod attack_profile;
mod dice;
mod dice_collection;
mod mutation_seed;
mod roll_instance;
mod static_modifier;

use attack_profile::AttackProfile;
use mutation_seed::MutationSeed;
use roll_instance::{RollInstance, RollInstanceBuilder};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Ruleset {
    DND5e,
    PF2e,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RollKind {
    Normal,
    Critical,
    Miss,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D20Value {
    Natural20,
    Normal,
    Natural1,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApplyTrait {
    Standard,
    Deadly { extra_sides: i32 },
    Fatal { upgraded_sides: i32 },
    OnMissOnly,
    OnCriticalOnly { doubles_with_crit: bool },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RollBehaviour {
    Standard,
    KeepHighFromX { extra_rolls: i32 },
    KeepLowFromX { extra_rolls: i32 },
}

// region: Private functions

/// Simulate a specified number of attack iterations and format the results as a DataFrame.
///
/// # Examples
/// ```
/// // ...
/// ```
fn evaluate_attack_profile(mut attack_profile: AttackProfile, number_turns: i32) -> DataFrame {
    let mut crit_counter: Vec<i32> = Vec::new();
    let mut hit_counter: Vec<i32> = Vec::new();
    let mut damage_counter: Vec<i32> = Vec::new();

    for _ in 0..number_turns {
        let (n_crits, n_hits, damage_rolled) = attack_profile.roll_turn();
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

/// Create a vector of AttackProfile structs corresponding to a vector of AC values.
///
/// Accepts a vector of target Armour Class values, and creates an attack profile for
/// each individual value.
///
/// # Examples
/// ```
/// // ...
/// ```
fn map_profiles_to_ac(
    ac_targets: Vec<i32>,
    hit_details: Vec<String>,
    weapon_details: Vec<String>,
    rule_set: Ruleset,
    seed: Option<u64>,
) -> Vec<AttackProfile> {
    let mut mut_seed = MutationSeed::new(seed);

    let profile_vector: Vec<AttackProfile> = ac_targets
        .into_iter()
        .map(|i| {
            let mut attack_profile = AttackProfile::new(i, rule_set);

            for (hit_value, dmg_value) in hit_details.iter().zip(weapon_details.iter()) {
                attack_profile.add_attack(hit_value, dmg_value, &mut mut_seed);
            }

            attack_profile
        })
        .collect();

    profile_vector
}

/// Extend the length of a vector by appending a new value the required number of times
///
/// # Examples
/// ```
/// let mut vector: Vec<String> = vec![String::from("A")];
/// let new_value = String::from("B");
///
/// resize_vector(&mut vector, new_value, 3);
/// ```
fn resize_vector(base_vector: &mut Vec<String>, new_value: String, iterations: usize) {
    // Extend the length of the input by appending the specified value a given number of times.
    let new_vector = vec![new_value; iterations];
    base_vector.extend(new_vector);
}

/// Collect the results of an attack profile simulation into a polars DataFrame
///
/// Records the target AC as a single integer, and vectors of the tallies for critical
/// hits, regular hits, and damage per turn for all turns simulated in the iteration.
/// Formats the results into a table in the format:
///
/// |Iteration|Target_AC|Number_hits|Number_crits|Total_damage|
/// |:---:|:---:|:---:|:---:|:---:|
/// |1|...|...|...|...|
/// |...|...|...|...|...|
/// |n|...|...|...|...|
///
/// # Examples
/// ```
/// let input_ac = 10;
/// let crit_counts = vec![0, 0, 1, 0];
/// let hit_counts = vec![0, 1, 1, 1];
/// let damage_results = vec![0, 1, 4, 1];
///
/// let df = results_to_dataframe(input_ac, crit_counts, hit_counts, damage_results);
/// ```
fn results_to_dataframe(
    ac_value: i32,
    crit_counter: Vec<i32>,
    hit_counter: Vec<i32>,
    damage_counter: Vec<i32>,
) -> DataFrame {
    let max_len: i32 = (crit_counter.len() as i32) + 1;
    let iteration_counter: Vec<i32> = (1..max_len).collect();
    let ac_counter: Vec<i32> = vec![ac_value; crit_counter.len()];

    // Create the DataFrame. This function cannot fail in this scope, so just unwrap and return.
    df!(
        "Iteration" => &iteration_counter,
        "Target_AC" => &ac_counter,
        "Number_hits" => &hit_counter,
        "Number_crits" => &crit_counter,
        "Total_damage" => &damage_counter
    )
    .unwrap()
}

// endregion:

// region: Public functions

/// Compare the lengths of two input vectors and extend the shorter instance
///
/// Identifies the shorter vector then pads the end with the terminal value of the
/// longer vector until both are of equal length. This acts as a shortcut for D&D-
/// style rules where there may be a single hit roll value for multiple attacks,
/// or for PF2e where successive attacks rolled with MAP use the same damage roll
/// on each attack.
///
/// # Examples
/// ```
/// let mut long_vector = vec![String::from("a"), String::from("b"), String::from("c")];
/// let mut short_vector = vec![String::from("a")];
///
/// equalise_input_vectors(&mut long_vector, &mut short_vector);
/// assert_eq!(vec!["a", "c", "c"], short_vector);
/// ```
pub fn equalise_input_vectors(first_vector: &mut Vec<String>, second_vector: &mut Vec<String>) {
    match first_vector.len().cmp(&second_vector.len()) {
        Ordering::Greater => {
            let length_diff = first_vector.len() - second_vector.len();
            let last_value = second_vector.last().unwrap().to_string();
            resize_vector(second_vector, last_value, length_diff);
        }
        Ordering::Less => {
            let length_diff = second_vector.len() - first_vector.len();
            let last_value = first_vector.last().unwrap().to_string();
            resize_vector(first_vector, last_value, length_diff);
        }
        Ordering::Equal => (),
    }
}

/// Partition the inputs over the range of AC values and simulate the attack turns.
///
/// Instantiates the attack simulation conditions into a vector mapping each specified
/// Armour Class value with the roll information. Runs the simulation in either single-
/// or multi-threaded mode, defaulting to a simple map/iter structure when no thread
/// information is provided.
///
/// # Examples
/// ```
/// // ...
/// ```
pub fn process_simulation(
    ac_targets: Vec<i32>,
    hit_details: Vec<String>,
    weapon_details: Vec<String>,
    ruleset: Ruleset,
    number_turns: i32,
    seed: Option<u64>,
    n_threads: Option<usize>,
) -> DataFrame {
    let profile_vector: Vec<AttackProfile> =
        map_profiles_to_ac(ac_targets, hit_details, weapon_details, ruleset, seed);

    let attack_results: Vec<LazyFrame> = match n_threads {
        Some(n) => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n)
                .build()
                .unwrap();

            pool.install(|| {
                profile_vector
                    .into_par_iter()
                    .map(|ap| evaluate_attack_profile(ap, number_turns).lazy())
                    .collect()
            })
        }
        None => profile_vector
            .into_iter()
            .map(|ap| evaluate_attack_profile(ap, number_turns).lazy())
            .collect(),
    };

    /* Documentation on the circumstances that cause the polars concat() function is lacking.
       For now just unwrap and return until I get a sighting of an error, at which point a
       separate function might be required.
    */
    let concat_args = UnionArgs {
        parallel: true,
        rechunk: true,
        to_supertypes: false,
        diagonal: false,
        from_partitioned_ds: false,
    };
    concat(attack_results, concat_args)
        .unwrap()
        .collect()
        .unwrap()
}

/// Summarise the raw simulation information to the average per-AC results
///
/// Takes a table representing all simulation data produced during the run
/// and reports the mean number of hits, critical hits, and damage for each
/// Armour Class value evaluated in the simulation run.
///
/// # Examples
/// ```
/// let input_df = df!(
///     "Target_AC" => &[10, 10, 10, 12, 12, 12],
///     "Number_hits" => &[1, 1, 1, 1, 0, 0],
///     "Number_crits" => &[1, 0, 0, 0, 0, 0],
///     "Total_damage" => &[6, 3, 3, 2, 0, 0],
/// ).unwrap()
///
/// let df = summarise_results(input_df);
/// ```
pub fn summarise_results(results_df: DataFrame) -> DataFrame {
    let agg_exprs = vec![
        col("Number_hits").mean().alias("Hits per round (mean)"),
        col("Number_crits")
            .mean()
            .alias("Critical hits per round (mean)"),
        col("Total_damage").mean().alias("Damage per round (mean)"),
    ];

    results_df
        .lazy()
        .group_by(["Target_AC"])
        .agg(agg_exprs)
        .sort(["Target_AC"], Default::default())
        .rename(["Target_AC"], ["Target AC"])
        .collect()
        .unwrap()
}

/// Write a DataFrame into the compressed parquet format.
///
/// # Examples
/// ```
/// let mut df = df!("Temp" => &[1, 2, 3]).unwrap();
///
/// write_to_parquet("example.parquet", &mut df)?;
/// ```
pub fn write_to_parquet(
    output_path: &str,
    file_content: &mut DataFrame,
) -> Result<(), Box<dyn Error>> {
    let error_msg = format!("Unable to write to output file path '{}'!", output_path);

    // Create the file path, and premature return if it fails
    let create_result = File::create(output_path);
    if create_result.is_err() {
        bail!(error_msg);
    }

    let target_file = create_result.unwrap();
    match ParquetWriter::new(target_file).finish(file_content) {
        Ok(_) => (),
        _ => bail!(error_msg),
    };

    Ok(())
}

// endregion:

#[cfg(test)]
mod tests {
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

    // region: dpr_simulator::evaluate_attack_profile tests

    #[test]
    fn test_evaluate_attack_profile() {
        let mut mut_seed = MutationSeed::new(None);

        let mut attack_profile = AttackProfile::new(1, Ruleset::DND5e);
        attack_profile.add_attack("1d1+1", "1d1+1", &mut mut_seed);

        let exp_df = df![
            "Iteration" => vec![1, 2, 3, 4, 5],
            "Target_AC" => vec![1; 5],
            "Number_hits" => vec![1; 5],
            "Number_crits" => vec![0; 5],
            "Total_damage" => vec![2; 5],
        ]
        .unwrap();

        let obs_df = evaluate_attack_profile(attack_profile, 5);
        dataframes_are_equal(exp_df, obs_df);
    }

    // endregion:

    // region: dpr_simulator::map_profiles_to_ac tests

    #[test]
    fn test_map_profiles_to_ac_single() {
        let mut mut_seed = MutationSeed::new(None);

        let mut attack_profile1 = AttackProfile::new(10, Ruleset::DND5e);
        attack_profile1.add_attack("1d4+1", "1d12+4", &mut mut_seed);

        let mut attack_profile2 = AttackProfile::new(15, Ruleset::DND5e);
        attack_profile2.add_attack("1d4+1", "1d12+4", &mut mut_seed);

        let exp_aps = vec![attack_profile1, attack_profile2];

        let obs_aps = map_profiles_to_ac(
            vec![10, 15],
            vec!["1d4+1".to_string()],
            vec!["1d12+4".to_string()],
            Ruleset::DND5e,
            None,
        );

        assert_eq!(exp_aps, obs_aps);
    }

    #[test]
    fn test_map_profiles_to_ac_multiple() {
        let mut mut_seed = MutationSeed::new(None);

        let mut attack_profile1 = AttackProfile::new(10, Ruleset::DND5e);
        attack_profile1.add_attack("1d4+1", "1d12+4", &mut mut_seed);
        attack_profile1.add_attack("1d5+1", "1d13+4", &mut mut_seed);

        let mut attack_profile2 = AttackProfile::new(15, Ruleset::DND5e);
        attack_profile2.add_attack("1d4+1", "1d12+4", &mut mut_seed);
        attack_profile2.add_attack("1d5+1", "1d13+4", &mut mut_seed);

        let exp_aps = vec![attack_profile1, attack_profile2];

        let obs_aps = map_profiles_to_ac(
            vec![10, 15],
            vec![String::from("1d4+1"), String::from("1d5+1")],
            vec![String::from("1d12+4"), String::from("1d13+4")],
            Ruleset::DND5e,
            None,
        );

        assert_eq!(exp_aps, obs_aps);
    }

    // endregion:

    // region: dpr_simulator::resize_vector tests

    #[test]
    fn test_resize_vector() {
        let exp_vector = create_string_vector(vec!["a", "b", "c", "d", "d"]);
        let mut input_vector = create_string_vector(vec!["a", "b", "c"]);

        resize_vector(&mut input_vector, "d".to_string(), 2);
        assert_eq!(exp_vector, input_vector);
    }

    #[test]
    fn test_resize_vector_no_change() {
        let exp_vector = create_string_vector(vec!["a", "b", "c"]);
        let mut input_vector = create_string_vector(vec!["a", "b", "c"]);

        resize_vector(&mut input_vector, "d".to_string(), 0);
        assert_eq!(exp_vector, input_vector);
    }

    // endregion:

    // region: dpr_simulator::results_to_dataframe tests

    #[test]
    fn test_results_to_dataframe() {
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

    // endregion:

    // region: dpr_simulator::equalise_input_vectors tests

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

    // endregion:

    // region: dpr_simulator::process_simulation tests

    #[test]
    fn test_process_simulation() {
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
            vec![String::from("1d1+1")],
            vec![String::from("1d1+1")],
            Ruleset::DND5e,
            5,
            None,
            None,
        );

        dataframes_are_equal(exp_df, obs_df);
    }

    #[test]
    fn test_process_simulation_multithreaded() {
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
            vec![String::from("1d1+1")],
            vec![String::from("1d1+1")],
            Ruleset::DND5e,
            5,
            None,
            Some(2),
        );

        dataframes_are_equal(exp_df, obs_df);
    }

    // endregion:

    // region: dpr_simulator::summarise_result tests

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

    // endregion:

    // region: dpr_simulator::write_to_parquet tests

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

    // endregion:
}
