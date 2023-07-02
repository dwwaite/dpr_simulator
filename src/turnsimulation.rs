use crate::AttackProfile;

pub fn process_simulation(attack_profile: &mut AttackProfile, ac_targets: Vec<i32>, number_turns: i32) -> () {

    for target_ac in &ac_targets {

        for i in 0..number_turns {

            let _ = attack_profile.roll_turn(&target_ac);
        }
    }
}
