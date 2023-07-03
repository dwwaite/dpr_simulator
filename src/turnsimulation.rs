use std::error::Error;
use polars::prelude::*;
use simple_error::bail;
use crate::AttackProfile;

fn vectors_to_dataframe(iteration_vector: Vec<i32>, ac_vector: Vec<i32>, damage_vector: Vec<i32>) -> Result<DataFrame, Box<dyn Error>> {

    let df = match df!(
        "Iteration" => iteration_vector,
        "AC" => ac_vector,
        "Damage" => damage_vector,
    ) {
        Ok(df) => df,
        _ => bail!("Error constructing dataframe of results!")
    };
    Ok(df)
}

pub fn process_simulation(attack_profile: &mut AttackProfile, ac_targets: Vec<i32>, number_turns: i32) -> Result<DataFrame, Box<dyn Error>> {

    let mut iter_vector: Vec<i32> = Vec::with_capacity(number_turns as usize);
    let mut ac_vector: Vec<i32> = Vec::with_capacity(number_turns as usize);
    let mut damage_vector: Vec<i32> = Vec::with_capacity(number_turns as usize);

    for target_ac in &ac_targets {

        for i in 0..number_turns {

            iter_vector.push(i);
            ac_vector.push(*target_ac);
            damage_vector.push(
                attack_profile.roll_turn(&target_ac)
            );
        }
    }

    let df = vectors_to_dataframe(iter_vector, ac_vector, damage_vector)?;
    Ok(df)
}
