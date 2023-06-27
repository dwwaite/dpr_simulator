use dpr_simulator::{TurnSimulation,Weapon};

fn main() {

    // Standard 1d8 weapon, and a 1st level PC with +3 in main stat
    let weapon = Weapon::new(1, 8);
    let mut ts = TurnSimulation::new(1, 5, 3, weapon);
    let target_ac = 12;

    let x = ts.roll(target_ac);
    println!("{:#?}", ts);
    println!("{}", x);
}
