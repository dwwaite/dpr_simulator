use dpr_simulator::{TurnSimulation,Weapon};

fn main() {

    // Standard 1d8 weapon, and a 1st level PC with +3 in main stat
    let weapon = Weapon::new(1, 8, 3);
    let mut ts = TurnSimulation::new(
        2,
        0,
        5,
        weapon,
        Weapon::create_empty()
    );
    let target_ac = 12;

    let _x = ts.roll_turn(target_ac);
}
