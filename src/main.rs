use dpr_simulator::TurnSimulation;

fn main() {
    println!("Hello, world!");

    let mut ts = TurnSimulation::new(1, 8, 0, 0);
    let target_ac = 12;

    let x = ts.roll(target_ac);
    println!("{:#?}", ts);
    println!("{}", x);
}
