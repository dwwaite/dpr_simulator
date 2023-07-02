use clap::Parser;
use std::error::Error;

mod weapon;
use weapon::Weapon;

mod attackprofile;
use attackprofile::AttackProfile;

mod turnsimulation;
use turnsimulation::process_simulation;

fn main() {

    let cli: Cli = Cli::parse();

    // Need to handle errors in this statement in the future...
    let (mainhand_attack, mainhand_weapon) = unpack_mh_details(&cli).unwrap();

    // Also handle the unwrap here.
    let (offhand_attack, offhand_weapon) = unpack_oh_details(&cli).unwrap();

    let mut attack_profile = AttackProfile::new(
        mainhand_attack,
        offhand_attack,
        cli.to_hit,
        mainhand_weapon,
        offhand_weapon,
    );

    let ac_targets: Vec<i32> = vec![12, 14, 16, 18, 20];

    process_simulation(&mut attack_profile, ac_targets, cli.number_turns);
}

fn unpack_mh_details(cli: &Cli) -> Result<(i32, Weapon), Box<dyn Error>> {

    let mainhand_attacks = &cli.mainhand_attacks;
    let mainhand_weapon = Weapon::from_notation_string(&cli.mainhand_weapon)?;

    Ok((*mainhand_attacks, mainhand_weapon))
}

fn unpack_oh_details(cli: &Cli) -> Result<(i32, Weapon), Box<dyn Error>> {

    let offhand_attacks: i32 = match cli.offhand_attacks {
        Some(o) => o,
        None => 0
    };
    let offhand_weapon = match &cli.offhand_weapon {
        Some(s) => Weapon::from_notation_string(&s)?,
        None => Weapon::create_empty(),
    };

    Ok((offhand_attacks, offhand_weapon))
}

#[derive(Parser)]
struct Cli {

    /// Number of main-hand attackes to make per turn
    #[arg(short = 'm', long, value_name = "MAINHAND ATTACKS")]
    mainhand_attacks: i32,

    /// Details of the mainhand weapon (e.g. 1d8+5)
    #[arg(short = 'w', long, value_name = "MAINHAND WEAPON")]
    mainhand_weapon: String,

    /// Number of off-hand attackes to make per turn (optional)
    #[arg(short = 'o', long, value_name = "OFFHAND ATTACKS")]
    offhand_attacks: Option<i32>,

    /// Details of the offhand weapon (optional)
    #[arg(long, value_name = "OFFHAND WEAPON")]
    offhand_weapon: Option<String>,

    /// To-Hit modifier
    #[arg(short, long, value_name = "TO HIT")]
    to_hit: i32,

    /// Number of turns to silmuate (default 1,000,000)
    #[arg(short, long, value_name = "NUMBER TURNS", default_value_t = 1_000_000)]
    number_turns: i32,
}
