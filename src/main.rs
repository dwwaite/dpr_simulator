use std::process::exit;

use clap::Parser;

mod attackprofile;

mod damageelement;
use damageelement::DamageElement;

mod turnsimulation;
use turnsimulation::process_simulation;

fn main() {
    // Entry point for the tool interface.

    let cli: Cli = Cli::parse();

    // Upack the optional off-hand options
    let offhand_attacks: i32 = match cli.offhand_attacks {
        Some(o) => o,
        None => 0,
    };
    let offhand_weapon: String = match cli.offhand_weapon {
        Some(s) => s,
        None => "".to_string(),
    };

    let _output_df = match process_simulation(
        vec![12, 14, 16, 18, 20],
        cli.to_hit,
        cli.mainhand_attacks,
        cli.mainhand_weapon,
        offhand_attacks,
        offhand_weapon,
        cli.number_turns,
    ) {
        Ok(df) => df,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}

#[derive(Parser)]
struct Cli {
    // Build the CLI input arguments and options.
    /// To-Hit modifier
    #[arg(short = 't', long, value_name = "TO HIT")]
    to_hit: i32,

    /// Path to save results (Apache parquet format)
    #[arg(short, long, value_name = "OUTPUT FILE")]
    output: String,

    /// Number of main-hand attackes to make per turn
    #[arg(short = 'm', long, value_name = "MAINHAND ATTACKS")]
    mainhand_attacks: i32,

    /// Details of the mainhand weapon (e.g. 1d8+5)
    #[arg(short = 'w', long, value_name = "MAINHAND WEAPON")]
    mainhand_weapon: String,

    /// Number of off-hand attackes to make per turn (optional)
    #[arg(long, value_name = "OFFHAND ATTACKS")]
    offhand_attacks: Option<i32>,

    /// Details of the offhand weapon (optional)
    #[arg(long, value_name = "OFFHAND WEAPON")]
    offhand_weapon: Option<String>,

    /// Number of turns to simulate (default 1,000,000)
    #[arg(short, long, value_name = "NUMBER TURNS", default_value_t = 1_000_000)]
    number_turns: i32,
}
