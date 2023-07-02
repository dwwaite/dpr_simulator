mod weapon;
use weapon::Weapon;

mod turnsimluation;
use turnsimluation::TurnSimulation;

fn main() {

    // Standard 1d8 weapon, and a 1st level PC with +3 in main stat
    let weapon = match Weapon::from_notation_string("1d8+3") {
        Ok(w) => w,
        _ => Weapon::create_empty()
    };

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
